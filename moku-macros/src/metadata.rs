use std::{collections::HashMap, fmt::DebugSet};

use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::{parse_quote, Ident, Item, ItemImpl, ItemMod};

pub struct State {
    ident: Ident,
    node: Ident,
    substate: Ident,
    superstates_enum: Ident,
    children: Vec<State>,
    autogen_enter: bool,
    imp: Option<ItemImpl>,
}

impl From<&Ident> for State {
    fn from(ident: &Ident) -> Self {
        Self {
            ident: ident.clone(),
            node: format_ident!("{ident}Node"),
            substate: format_ident!("{ident}Substate"),
            superstates_enum: format_ident!("{ident}Superstates"),
            children: Vec::new(),
            autogen_enter: false,
            imp: None,
        }
    }
}

impl State {
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
        format!("{}", self.ident) + &self.state_chart_children(0)
    }

    /// Helper function for formatting the children of a `state_chart`.
    fn state_chart_children(&self, level: usize) -> String {
        if let Some((last, firsts)) = self.children.split_last() {
            let mut acc = String::new();
            for child in firsts {
                acc += &child.state_chart_acc(level + 1, false);
            }
            acc + &last.state_chart_acc(level + 1, true)
        } else {
            String::new()
        }
    }

    /// Helper function for recursively formatting `state_chart`.
    fn state_chart_acc(&self, level: usize, last: bool) -> String {
        let pad = " ".repeat(level * 3);
        let vert = if last { '\u{02514}' } else { '\u{0251C}' };
        format!("\n{pad}{vert}\u{02500} {}", self.ident) + &self.state_chart_children(level)
    }

    /// Create a copy that does not include an ItemImpl or children.
    fn shallow_copy(&self) -> Self {
        Self {
            ident: self.ident.clone(),
            node: self.node.clone(),
            substate: self.substate.clone(),
            superstates_enum: self.superstates_enum.clone(),
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
        fun(self, &ancestors);
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
    pub top_state: State,
    pub states: HashMap<Ident, State>,
    pub machine_mod: ItemMod,
    pub module: ItemMod,
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
            .expect(&format!("add_state_impl: could not find state {}", ident))
            .imp = Some(imp);
    }

    /// Add a parent-child relation to the state graph, while detecting state graph cycles.
    pub fn add_relation(&mut self, parent: &Ident, child: &Ident) -> Result<(), syn::Error> {
        let mut child = self
            .states
            .remove(child)
            .expect(&format!("add_relation: could not find child {}", child));

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
                child.state_chart_acc(0, true),
            ),
        ))
    }

    /// Write the state machine and return the complete state_machine module.
    pub fn write_state_machine(mut self) -> ItemMod {
        self.write_state_chart();
        self.write_state_enum();
        self.write_machine();
        self.write_builder();
        self.write_states();

        // put machine_mod back into the main module
        self.module
            .content
            .as_mut()
            .expect("write_state_machine: no content in module")
            .1
            .push(Item::Mod(self.machine_mod));

        self.module
    }

    /// Get a mutable reference to the contents of the machine module.
    fn machine_mod_contents(&mut self) -> &mut Vec<Item> {
        &mut self
            .machine_mod
            .content
            .as_mut()
            .expect("push_to_machine_mod: no content in module")
            .1
    }

    /// Push an Item into the contents of the machine module.
    fn push_to_machine_mod(&mut self, item: Item) {
        self.machine_mod_contents().push(item)
    }

    /// Get an Iterator of all states in this machine.
    fn all_states(&self) -> impl Iterator<Item = Ident> {
        [self.top_state.ident.clone()]
            .into_iter()
            .chain(self.top_state.descendents().into_iter())
    }

    /// Write the state chart to the machine module.
    fn write_state_chart(&mut self) {
        let ident = format_ident!("{}_STATE_CHART", self.name.to_string().to_case(Case::Upper));
        let chart = self.top_state.state_chart();

        // TODO remove
        self.push_to_machine_mod(parse_quote! {
            use crate as moku;
        });

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
          impl moku::StateEnum for #ident {}
        });
    }

    /// Write the StateMachine to the machine module.
    fn write_machine(&mut self) {
        let ident = format_ident!("{}Machine", self.name);
        let state_enum = &self.state_enum;
        let top_state = &self.top_state.ident;
        let top_substate = &self.top_state.substate;

        self.push_to_machine_mod(parse_quote! {
            pub struct #ident {
                top_node: moku::internal::TopNode<#state_enum, super::#top_state, #top_substate>,
            }
        });

        let state_enum = &self.state_enum;
        let top_state = &self.top_state.ident;

        self.push_to_machine_mod(parse_quote! {
            impl moku::StateMachine<#state_enum, super::#top_state> for #ident {
                fn update(&mut self) {
                    self.top_node.update()
                }

                fn top_down_update(&mut self) {
                    self.top_node.top_down_update()
                }

                fn transition(&mut self, target: #state_enum) {
                    self.top_node.transition(target);
                }

                fn state(&self) -> #state_enum {
                    self.top_node.state()
                }

                fn name(&self) -> &str {
                    self.top_node.name()
                }

                fn set_name(&mut self, name: String) {
                    self.top_node.set_name(name)
                }

                fn state_matches(&self, state: #state_enum) -> bool {
                    self.top_node.state_matches(state)
                }

                fn top_ref(&self) -> &super::#top_state {
                    &self.top_node.node.state
                }

                fn top_mut(&mut self) -> &mut super::#top_state {
                    &mut self.top_node.node.state
                }
            }
        });

        for state in self.all_states() {
            let state_enum = &self.state_enum;

            self.push_to_machine_mod(parse_quote! {
                impl moku::StateRef<#state_enum, super::#state> for #ident {
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

        self.push_to_machine_mod(parse_quote! {
            pub struct #ident {
                top_state: super::#top_state,
                name: Option<String>,
            }
        });

        let state_enum = &self.state_enum;
        let top_state = &self.top_state.ident;
        let name = self.name.to_string();

        self.push_to_machine_mod(parse_quote! {
            impl moku::StateMachineBuilder<#state_enum, super::#top_state, #machine_ident> for #ident {
                fn new(top_state: super::#top_state) -> Self {
                    Self {
                        top_state,
                        name: None,
                    }
                }

                fn name(mut self, name: &str) -> Self {
                    self.name = Some(name.to_owned());
                    self
                }

                fn build(self) -> #machine_ident {
                    #machine_ident::new(moku::internal::TopNode::new(
                        self.top_state,
                        self.name.unwrap_or_else(|| String::from(#name)),
                    ))
                }
            }
        });
    }

    /// Write the auto-generated elements for each State to their impls and the machine module.
    fn write_states(&mut self) {
        let items: Vec<Item> = Vec::new();
        let state_enum = &self.state_enum.clone();
        let machine_mod = &self.machine_mod.ident.clone();

        self.top_state.for_each_state(|state, ancestors| {
            let imp = state.imp.as_mut().expect(&format!(
                "write_states: missing State impl for {}",
                state.ident
            ));

            if state.autogen_enter {
                imp.items.push(parse_quote! {
                    fn enter(superstates: &mut Self::Superstates<'_>) -> StateEntry<Self, #state_enum> {
                        StateEntry::State(Self {})
                    }
                });
            }

            let superstates = &state.superstates_enum;

            imp.items.push(parse_quote! {
                type Superstates<'a> = #machine_mod::#superstates<'a>;
            });

            // TODO populate items
        });

        self.machine_mod_contents().extend(items);
    }
}
