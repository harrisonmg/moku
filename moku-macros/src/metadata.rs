use std::{collections::HashMap, fmt::DebugSet};

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Ident, Item, ItemImpl, ItemMod};

pub struct State {
    ident: Ident,
    children: Vec<State>,
    autogen_enter: bool,
    imp: Option<ItemImpl>,
}

impl From<&Ident> for State {
    fn from(ident: &Ident) -> Self {
        Self {
            ident: ident.clone(),
            children: Vec::new(),
            autogen_enter: false,
            imp: None,
        }
    }
}

impl State {
    /// Get the Ident for this State's Node.
    fn node_ident(&self) -> Ident {
        format_ident!("{}Node", self.ident)
    }

    /// Get the Ident for this State's SubstateEnum.
    fn substate_enum_ident(&self) -> Ident {
        format_ident!("{}Substate", self.ident)
    }

    /// Get the Ident for this State's Superstates.
    fn superstates_ident(&self) -> Ident {
        format_ident!("{}Superstates", self.ident)
    }

    /// Add a child to this State or one of its descendents. Returns the child if no parent is
    /// found.
    fn add_child(&mut self, mut child: State, parent: &Ident) -> Option<State> {
        if self.ident == *parent {
            self.children.push(child);
            None
        } else {
            for state in &mut self.children {
                if let Some(reject) = state.add_child(child, parent) {
                    child = reject;
                } else {
                    return None;
                }
            }

            Some(child)
        }
    }

    /// Generate a simple text state chart of this State and its children.
    fn state_chart(&self) -> String {
        format!("{}", self.ident) + &self.state_chart_children(&mut Vec::new())
    }

    /// Helper function for formatting the children of a `state_chart`.
    fn state_chart_children(&self, levels: &mut Vec<bool>) -> String {
        if let Some((last, firsts)) = self.children.split_last() {
            let mut acc = String::new();

            for child in firsts {
                acc += &child.state_chart_acc(levels, false);
            }

            acc + &last.state_chart_acc(levels, true)
        } else {
            String::new()
        }
    }

    /// Helper function for recursively formatting `state_chart`.
    fn state_chart_acc(&self, levels: &mut Vec<bool>, last: bool) -> String {
        let mut pad = String::new();
        for bar in levels.iter() {
            if *bar {
                pad += "\u{02502}  ";
            } else {
                pad += "   ";
            }
        }

        let vert = if last { '\u{02514}' } else { '\u{0251C}' };

        levels.push(!last);
        let ret =
            format!("\n{pad}{vert}\u{02500} {}", self.ident) + &self.state_chart_children(levels);
        levels.pop();

        ret
    }

    /// Create a copy that does not include an ItemImpl or children.
    fn shallow_copy(&self) -> Self {
        Self {
            ident: self.ident.clone(),
            children: Vec::new(),
            autogen_enter: self.autogen_enter,
            imp: None,
        }
    }

    /// Execute a function for this state and each substate, with an list of shallow-copy
    /// ancestors.
    fn for_each_state<F: FnMut(&mut State, &Vec<State>)>(&mut self, mut fun: F) {
        self.for_each_state_acc(&mut fun, &mut Vec::new());
    }

    /// Helper to accumulate ancestors for `for_each_state`.
    fn for_each_state_acc<F: FnMut(&mut State, &Vec<State>)>(
        &mut self,
        mut fun: &mut F,
        mut ancestors: &mut Vec<State>,
    ) {
        fun(self, ancestors);
        ancestors.push(self.shallow_copy());
        for child in &mut self.children {
            child.for_each_state_acc(fun, ancestors);
        }
        ancestors.pop();
    }

    /// Get all descendents of this State.
    fn descendents(&self) -> Vec<Ident> {
        let mut res = Vec::new();
        for child in &self.children {
            child.descendents_acc(&mut res);
        }
        res
    }

    /// Helper to accumulate descendent list for `descendents`.
    fn descendents_acc(&self, acc: &mut Vec<Ident>) {
        acc.push(self.ident.clone());
        for child in &self.children {
            child.descendents_acc(acc);
        }
    }
}

pub struct Metadata {
    pub name: Ident,
    pub state_enum: Ident,
    pub event: TokenStream,
    pub top_state: State,
    pub states: HashMap<Ident, State>,
    pub machine_mod: ItemMod,
    pub main_mod: ItemMod,
}

impl Metadata {
    /// Add a state machine state.
    pub fn add_state(&mut self, ident: &Ident, autogen_enter: bool, imp: ItemImpl) {
        let mut state: State = ident.into();
        state.autogen_enter = autogen_enter;
        state.imp = Some(imp);
        self.states.insert(ident.clone(), state);
    }

    /// Add the State impl item to a state.
    pub fn add_state_impl(&mut self, ident: &Ident, imp: ItemImpl) {
        self.states
            .get_mut(ident)
            .unwrap_or_else(|| panic!("add_state_impl: could not find state {}", ident))
            .imp = Some(imp);
    }

    /// Add a parent-child relation to the state graph, while detecting state graph cycles.
    pub fn add_relation(&mut self, parent: &Ident, child: &Ident) -> Result<(), syn::Error> {
        let mut child = self
            .states
            .remove(child)
            .unwrap_or_else(|| panic!("add_relation: could not find child {}", child));

        for state in self.states.values_mut().chain([&mut self.top_state]) {
            if let Some(reject) = state.add_child(child, parent) {
                child = reject;
            } else {
                return Ok(());
            }
        }

        // we didn't find the parent, this is a cycle
        Err(syn::Error::new(
            parent.span(),
            format!(
                "state graph cycle detected in children of {parent}:\n{parent}{}",
                child.state_chart_acc(&mut Vec::new(), true),
            ),
        ))
    }

    /// Write the state machine and return the complete main module.
    pub fn write_state_machine(mut self) -> ItemMod {
        self.write_state_chart();
        self.write_state_enum();
        self.write_machine();
        self.write_builder();
        self.write_states();

        // put the machine_mod back into the main module
        let main_mod_content = &mut self
            .main_mod
            .content
            .as_mut()
            .expect("main_mod_content: no content in module")
            .1;

        main_mod_content.push(Item::Mod(self.machine_mod));

        self.main_mod
    }

    /// Get a mutable reference to the contents of the machine module.
    fn machine_mod_content(&mut self) -> &mut Vec<Item> {
        &mut self
            .machine_mod
            .content
            .as_mut()
            .expect("machine_mod_content: no content in module")
            .1
    }

    /// Push an Item into the contents of the machine module.
    fn push_to_machine_mod(&mut self, item: Item) {
        self.machine_mod_content().push(item)
    }

    /// Get an Iterator of all states in this machine.
    fn all_states(&self) -> impl Iterator<Item = Ident> {
        [self.top_state.ident.clone()]
            .into_iter()
            .chain(self.top_state.descendents())
    }

    /// Write the state chart to the machine module.
    fn write_state_chart(&mut self) {
        let ident = format_ident!(
            "{}_STATE_CHART",
            self.name.to_string().to_case(Case::UpperSnake)
        );
        let chart = self.top_state.state_chart();

        self.push_to_machine_mod(parse_quote! {
            pub const #ident: &str = #chart;
        });
    }

    /// Write the StateEnum to the machine module.
    fn write_state_enum(&mut self) {
        let ident = &self.state_enum;
        let states = self.all_states();

        self.push_to_machine_mod(parse_quote! {
          #[derive(Debug, Clone, Copy, PartialEq, Eq)]
          pub enum #ident {
              #(#states,)*
          }
        });

        let ident = &self.state_enum;

        self.push_to_machine_mod(parse_quote! {
          impl ::moku::StateEnum for #ident {}
        });
    }

    /// Write the StateMachine to the machine module.
    fn write_machine(&mut self) {
        let ident = format_ident!("{}Machine", self.name);
        let state_enum = self.state_enum.clone();
        let event = self.event.clone();
        let top_state = self.top_state.ident.clone();
        let top_substate = self.top_state.substate_enum_ident();

        self.push_to_machine_mod(parse_quote! {
            pub struct #ident {
                top_node: ::moku::internal::TopNode<#state_enum, #event, super::#top_state, #top_substate>,
            }
        });

        self.push_to_machine_mod(parse_quote! {
            impl #ident {
                fn new(top_node: ::moku::internal::TopNode<#state_enum, #event, super::#top_state, #top_substate>) -> Self {
                    let mut new = Self { top_node };
                    new.top_node.init();
                    new
                }
            }
        });

        let set_name = if cfg!(feature = "std") {
            quote! {
                fn set_name(&mut self, name: String) {
                    self.top_node.set_name(name)
                }
            }
        } else {
            TokenStream::new()
        };

        self.push_to_machine_mod(parse_quote! {
            impl ::moku::StateMachine<#state_enum, #event, super::#top_state> for #ident {
                fn update(&mut self) {
                    self.top_node.update()
                }

                fn top_down_update(&mut self) {
                    self.top_node.top_down_update()
                }

                fn transition(&mut self, target: #state_enum) {
                    self.top_node.transition(target, false);
                }

                fn state(&self) -> #state_enum {
                    self.top_node.state()
                }

                fn name(&self) -> &str {
                    self.top_node.name()
                }

                #set_name

                fn state_matches(&self, state: #state_enum) -> bool {
                    self.top_node.state_matches(state)
                }

                fn top_ref(&self) -> &super::#top_state {
                    &self.top_node.node.state
                }

                fn top_mut(&mut self) -> &mut super::#top_state {
                    &mut self.top_node.node.state
                }

                fn handle_event(&mut self, event: &#event) {
                    self.top_node.handle_event(event)
                }
            }
        });

        for state in self.all_states() {
            let state_enum = &self.state_enum;
            let event = &self.event;

            self.push_to_machine_mod(parse_quote! {
                impl ::moku::StateRef<#state_enum, #event, super::#state> for #ident {
                    fn state_ref(&self) -> Option<&super::#state> {
                        self.top_node.node.state_ref()
                    }

                    fn state_mut(&mut self) -> Option<&mut super::#state> {
                        self.top_node.node.state_mut()
                    }
                }
            });
        }
    }

    /// Write the StateMachineBuilder to the machine module.
    fn write_builder(&mut self) {
        let ident = format_ident!("{}MachineBuilder", self.name);
        let machine_ident = format_ident!("{}Machine", self.name);
        let top_state = &self.top_state.ident;

        let name_field = if cfg!(feature = "std") {
            quote! {
                name: Option<String>,
            }
        } else {
            TokenStream::new()
        };

        self.push_to_machine_mod(parse_quote! {
            pub struct #ident {
                top_state: super::#top_state,
                #name_field
            }
        });

        let name_field = if cfg!(feature = "std") {
            quote! {
                name: None,
            }
        } else {
            TokenStream::new()
        };

        let name_setter = if cfg!(feature = "std") {
            quote! {
                fn name(mut self, name: String) -> Self {
                    self.name = Some(name);
                    self
                }
            }
        } else {
            TokenStream::new()
        };

        let name = self.name.to_string();

        let name_arg = if cfg!(feature = "std") {
            quote! {
                self.name.unwrap_or_else(|| String::from(#name)),
            }
        } else {
            quote! {
                #name
            }
        };

        let state_enum = &self.state_enum;
        let event = &self.event;
        let top_state = &self.top_state.ident;

        self.push_to_machine_mod(parse_quote! {
            impl ::moku::StateMachineBuilder<#state_enum, #event, super::#top_state, #machine_ident> for #ident {
                fn new(top_state: super::#top_state) -> Self {
                    Self {
                        top_state,
                        #name_field
                    }
                }

                #name_setter

                fn build(self) -> #machine_ident {
                    #machine_ident::new(::moku::internal::TopNode::new(
                        self.top_state,
                        #name_arg
                    ))
                }
            }
        });
    }

    /// Write the auto-generated elements for each State to their impls and the machine module.
    fn write_states(&mut self) {
        let mut items: Vec<Item> = Vec::new();
        let state_enum = self.state_enum.clone();
        let event = &self.event;
        let machine_mod = self.machine_mod.ident.clone();
        let mut state_impls = Vec::new();
        let all_states: Vec<_> = self.all_states().collect();

        self.top_state.for_each_state(|state, ancestors| {
            let is_top_state = ancestors.is_empty();

            let parent_superstates = match ancestors.last() {
                None => quote! { ::moku::NoSuperstates },
                Some(parent) => parent.superstates_ident().into_token_stream(),
            };

            // State enter and Superstates
            if !is_top_state {
                let mut imp = state.imp.take().unwrap_or_else(|| panic!(
                    "write_states: missing State impl for {}",
                    state.ident
                ));

                if state.autogen_enter {
                    imp.items.push(parse_quote! {
                        fn enter(_superstates: &mut Self::Superstates<'_>) -> ::moku::StateEntry<Self, #state_enum> {
                            ::moku::StateEntry::State(Self {})
                        }
                    });
                }

                imp.items.push(parse_quote! {
                    type Superstates<'a> = #machine_mod::#parent_superstates<'a>;
                });

                state_impls.push(imp);
            }

            // Node
            let state_ident = state.ident.clone();
            let node = state.node_ident();
            let substate = state.substate_enum_ident();

            items.push(parse_quote! {
               type #node = ::moku::internal::Node<#state_enum, #event, super::#state_ident, #substate>;
            });

            // Superstates
            let superstates = state.superstates_ident();

            let ancestor_idents = ancestors.iter().map(|anc| &anc.ident);
            let ancestor_idents_snake: Vec<_> = ancestors.iter().map(|anc|
                    Ident::new(&anc.ident.to_string().to_case(Case::Snake), Span::call_site())
                ).collect();
            let state_ident_snake = Ident::new(&state_ident.to_string().to_case(Case::Snake), Span::call_site());

            items.push(parse_quote! {
               pub struct #superstates<'a> {
                   #(pub #ancestor_idents_snake: &'a mut super::#ancestor_idents,)*
                   pub #state_ident_snake: &'a mut super::#state_ident,
               }
            });

            items.push(parse_quote! {
               impl<'a> #superstates<'a> {
                   pub fn new(state: &'a mut super::#state_ident, superstates: &'a mut #parent_superstates) -> Self {
                       Self {
                           #(#ancestor_idents_snake: superstates.#ancestor_idents_snake,)*
                           #state_ident_snake: state,
                       }
                   }
               }
            });

            // SubstateEnum
            let children: Vec<_> = state.children.iter().map(|child| &child.ident).collect();
            let children_nodes: Vec<_> = state.children.iter().map(|child| child.node_ident()).collect();
            let descendents = state.descendents();

            items.push(parse_quote! {
              enum #substate {
                  None,
                  #(#children(#children_nodes),)*
              }
            });

            let is_leaf_state = children.is_empty();

            if is_leaf_state {
                items.push(parse_quote! {
                    impl ::moku::internal::SubstateEnum<#state_enum, #event, super::#state_ident> for #substate {
                        fn none_variant() -> Self {
                            Self::None
                        }

                        fn this_state() -> #state_enum {
                            #state_enum::#state_ident
                        }

                        fn is_state(state: #state_enum) -> bool {
                            matches!(state, #state_enum::#state_ident)
                        }
                    }
                });
            } else {
                let children_and_descendents = state.children.iter().map(|child| {
                    let mut res = child.descendents();
                    res.push(child.ident.clone());
                    res
                });

                let is_ancestor = if is_top_state {
                    quote! {
                        fn is_ancestor(state: #state_enum) -> bool {
                            !matches!(state, #state_enum::#state_ident)
                        }
                    }
                } else {
                    quote! {
                        fn is_ancestor(state: #state_enum) -> bool {
                            matches!(state, #(#state_enum::#descendents)|*)
                        }
                    }
                };

                items.push(parse_quote! {
                    impl ::moku::internal::SubstateEnum<#state_enum, #event, super::#state_ident> for #substate {
                        fn none_variant() -> Self {
                            Self::None
                        }

                        fn this_state() -> #state_enum {
                            #state_enum::#state_ident
                        }

                        fn is_state(state: #state_enum) -> bool {
                            matches!(state, #state_enum::#state_ident)
                        }

                        fn current_state(&self) -> #state_enum {
                            match self {
                                Self::None => #state_enum::#state_ident,
                                #(Self::#children(node) => node.current_state(),)*
                            }
                        }

                        #is_ancestor

                        fn update(
                            &mut self,
                            state: &mut super::#state_ident,
                            superstates: &mut <super::#state_ident as ::moku::State<#state_enum, #event>>::Superstates<'_>,
                        ) -> Option<#state_enum> {
                            match self {
                                Self::None => None,
                                #(Self::#children(node) => node.update(&mut #superstates::new(state, superstates)),)*
                            }
                        }

                        fn update_in_need(
                            &mut self,
                            state: &mut super::#state_ident,
                            superstates: &mut <super::#state_ident as ::moku::State<#state_enum, #event>>::Superstates<'_>,
                        ) -> Option<#state_enum> {
                            match self {
                                Self::None => None,
                                #(Self::#children(node) => node.update_in_need(&mut #superstates::new(state, superstates)),)*
                            }
                        }

                        fn top_down_update(
                            &mut self,
                            state: &mut super::#state_ident,
                            superstates: &mut <super::#state_ident as ::moku::State<#state_enum, #event>>::Superstates<'_>,
                        ) -> Option<#state_enum> {
                            match self {
                                Self::None => None,
                                #(Self::#children(node) => {
                                    node.top_down_update(&mut #superstates::new(state, superstates))
                                })*
                            }
                        }

                        fn top_down_update_in_need(
                            &mut self,
                            state: &mut super::#state_ident,
                            superstates: &mut <super::#state_ident as ::moku::State<#state_enum, #event>>::Superstates<'_>,
                        ) -> Option<#state_enum> {
                            match self {
                                Self::None => None,
                                #(Self::#children(node) => {
                                    node.top_down_update_in_need(&mut #superstates::new(state, superstates))
                                })*
                            }
                        }

                        fn clear_top_down_updated(&mut self) {
                            match self {
                                Self::None => (),
                                #(Self::#children(node) => node.clear_top_down_updated(),)*
                            }
                        }

                        fn exit(
                            &mut self,
                            state: &mut super::#state_ident,
                            superstates: &mut <super::#state_ident as ::moku::State<#state_enum, #event>>::Superstates<'_>,
                            in_update: bool,
                        ) -> Option<#state_enum> {
                            let old_state = core::mem::replace(self, Self::None);
                            match old_state {
                                Self::None => None,
                                #(Self::#children(node) => node.exit(
                                        &mut #superstates::new(state, superstates),
                                        in_update,
                                ),)*
                            }
                        }

                        fn transition(
                            &mut self,
                            target: #state_enum,
                            state: &mut super::#state_ident,
                            superstates: &mut <super::#state_ident as ::moku::State<#state_enum, #event>>::Superstates<'_>,
                            in_update: bool,
                        ) -> ::moku::internal::TransitionResult<#state_enum> {
                            match self {
                                Self::None => ::moku::internal::TransitionResult::MoveUp,
                                #(Self::#children(node) => {
                                    node.transition(target, &mut #superstates::new(state, superstates), in_update)
                                })*
                            }
                        }

                        fn enter_substate_towards(
                            &mut self,
                            target: #state_enum,
                            state: &mut super::#state_ident,
                            superstates: &mut <super::#state_ident as ::moku::State<#state_enum, #event>>::Superstates<'_>,
                            in_update: bool,
                        ) -> Option<#state_enum> {
                            match target {
                                #(
                                    #(#state_enum::#children_and_descendents)|* => {
                                        match #children_nodes::enter(
                                            &mut #superstates::new(state, superstates),
                                            in_update,
                                        ) {
                                            ::moku::internal::NodeEntry::Node(node) => {
                                                *self = Self::#children(node);
                                                None
                                            }
                                            ::moku::internal::NodeEntry::Transition(new_target) => Some(new_target),
                                        }
                                    }
                                )*
                                _ => unreachable!(),
                            }
                        }

                        fn state_matches(&self, state: #state_enum) -> bool {
                            Self::is_state(state)
                                || match self {
                                    Self::None => false,
                                    #(Self::#children(node) => node.state_matches(state),)*
                                }
                        }

                        fn handle_event(
                            &mut self,
                            event: &#event,
                            state: &mut super::#state_ident,
                            superstates: &mut <super::#state_ident as ::moku::State<#state_enum, #event>>::Superstates<'_>,
                        ) -> ::moku::EventResponse<#state_enum> {
                            match self {
                                Self::None => ::moku::EventResponse::Defer,
                                #(Self::#children(node) => node.handle_event(
                                        event,
                                        &mut #superstates::new(state, superstates),
                                ),)*
                            }
                        }
                    }
                });
            }

            // StateRef
            for other_state in &all_states {
                items.push(
                    if *other_state == state_ident {
                        parse_quote! {
                            impl ::moku::StateRef<#state_enum, #event, super::#other_state> for #node {
                                fn state_ref(&self) -> Option<&super::#other_state> {
                                    Some(&self.state)
                                }

                                fn state_mut(&mut self) -> Option<&mut super::#other_state> {
                                    Some(&mut self.state)
                                }
                            }
                        }
                    } else if descendents.contains(other_state) {
                        parse_quote! {
                            impl ::moku::StateRef<#state_enum, #event, super::#other_state> for #node {
                                fn state_ref(&self) -> Option<&super::#other_state> {
                                    match &self.substate {
                                        #substate::None => None,
                                        #(#substate::#children(node) => node.state_ref(),)*
                                    }
                                }

                                fn state_mut(&mut self) -> Option<&mut super::#other_state> {
                                    match &mut self.substate {
                                        #substate::None => None,
                                        #(#substate::#children(node) => node.state_mut(),)*
                                    }
                                }
                            }
                        }
                    } else {
                        parse_quote! {
                            impl ::moku::StateRef<#state_enum, #event, super::#other_state> for #node {
                                fn state_ref(&self) -> Option<&super::#other_state> {
                                    None
                                }

                                fn state_mut(&mut self) -> Option<&mut super::#other_state> {
                                    None
                                }
                            }
                        }
                    }
                );
            }
        });

        let main_mod_content = &mut self
            .main_mod
            .content
            .as_mut()
            .expect("main_mod_content: no content in module")
            .1;

        for imp in state_impls {
            main_mod_content.push(Item::Impl(imp));
        }

        self.machine_mod_content().extend(items);
    }
}
