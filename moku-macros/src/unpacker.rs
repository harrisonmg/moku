use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    spanned::Spanned, Attribute, GenericArgument, ImplItem, Item, ItemImpl, ItemMod, ItemStruct,
    Meta, PathArguments, Type, TypePath,
};

use crate::{
    metadata::{Metadata, State},
    util::{filter_attributes, path_matches},
};

/// Collect and validate Metadata about the structure of a `state_machine` module and the usage of attributes.
pub fn build_metadata(name: Ident, module: ItemMod) -> Result<Metadata, syn::Error> {
    let mut unpacker = Unpacker::new(name, module);
    unpacker.unpack()?;
    unpacker.check_state_defs();
    unpacker.validate_associated_types()?;
    unpacker.validate_enter_defs()?;
    let top_state = unpacker.get_top_state()?;
    unpacker.validate_superstates(top_state)?;
    unpacker.build_metadata()
}

struct UnpackedState {
    ident: Ident,
    superstate: Ident,
    imp: ItemImpl,
    superstate_span: Span,
    def_found: bool,
    has_fields: bool,
    autogen_enter: bool,
}

struct Unpacker {
    name: Ident,
    main_mod: ItemMod,
    machine_mod: Option<ItemMod>,
    event: Option<Ident>,
    top_state: Option<Ident>,
    top_state_impl: Option<ItemImpl>,
    states: Vec<UnpackedState>,
    state_idents: HashSet<Ident>,
    structs: HashMap<Ident, bool>,
    error: Option<syn::Error>,
}

impl Unpacker {
    fn new(name: Ident, main_mod: ItemMod) -> Self {
        Self {
            name,
            main_mod,
            machine_mod: None,
            event: None,
            top_state: None,
            top_state_impl: None,
            states: Vec::new(),
            state_idents: HashSet::new(),
            structs: HashMap::new(),
            error: None,
        }
    }

    /// Build Metadata from the info collected by an Unpacker.
    fn build_metadata(mut self) -> Result<Metadata, syn::Error> {
        let (event, event_local) = self.take_event();
        let mut metadata = Metadata {
            event,
            event_local,
            top_state: self.get_top_state()?.into(),
            top_state_impl: self.top_state_impl.take(),
            machine_mod: self.take_machine_mod()?,
            name: self.name,
            states: HashMap::new(),
            main_mod: self.main_mod,
        };

        let relations: Vec<_> = self
            .states
            .into_iter()
            .map(|state| {
                metadata.add_state(&state.ident, state.autogen_enter, state.imp);
                (state.ident, state.superstate)
            })
            .collect();

        for (state, superstate) in relations {
            metadata.add_relation(&superstate, &state)?;
        }

        Ok(metadata)
    }

    /// Unpack each item in the state_machine module's content.
    fn unpack(&mut self) -> Result<(), syn::Error> {
        if let Some(mut content) = self.main_mod.content.take() {
            if !content.1.is_empty() {
                let items: Vec<_> = content.1.drain(..).collect();

                for item in &items {
                    // first pass to check for StateMachineEvent type
                    if let Item::Impl(imp) = item {
                        self.find_event(imp)
                    }

                    // stop if we encounter an issue
                    if let Some(error) = self.error.take() {
                        return Err(error);
                    }
                }

                for item in items {
                    if let Some(item) = match item {
                        Item::Struct(def) => self.unpack_struct(def),
                        Item::Mod(module) => self.unpack_mod(module),
                        Item::Impl(imp) => self.unpack_impl(imp),
                        _ => Some(item),
                    } {
                        // restore the items that we won't need to touch
                        content.1.push(item);
                    }

                    // stop if we encounter an issue
                    if let Some(error) = self.error.take() {
                        return Err(error);
                    }
                }

                self.main_mod.content = Some(content);
                return Ok(());
            }
        }

        // fallthrough error for above validation
        let msg = format!(
            "a `moku::state_machine` module must be inline with its attribute, try
```
#[moku::state_machine]
mod {} {{
    ...
}}
```",
            self.main_mod.ident
        );
        Err(syn::Error::new(self.main_mod.span(), msg))
    }

    /// Check each found State struct definition with its struct definition.
    fn check_state_defs(&mut self) {
        for state in &mut self.states {
            match self.structs.remove(&state.ident) {
                Some(has_fields) => {
                    state.def_found = true;
                    state.has_fields = has_fields;
                }
                None => {
                    state.def_found = false;
                }
            };
        }
    }

    /// Validate that associated types are not manually defined for any Substate.
    fn validate_associated_types(&self) -> Result<(), syn::Error> {
        for state in &self.states {
            for item in &state.imp.items {
                if let ImplItem::Type(ty) = item {
                    if ty.ident == "Context" || ty.ident == "State" || ty.ident == "Event" {
                        return Err(syn::Error::new(
                            ty.span(),
                            format!(
                                "the `Substate::{}` associated type must not be manually defined",
                                ty.ident
                            ),
                        ));
                    }
                }
            }
        }

        // Also check TopState impl
        if let Some(ref imp) = self.top_state_impl {
            for item in &imp.items {
                if let ImplItem::Type(ty) = item {
                    if ty.ident == "State" || ty.ident == "Event" {
                        return Err(syn::Error::new(
                            ty.span(),
                            format!(
                                "the `TopState::{}` associated type must not be manually defined",
                                ty.ident
                            ),
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate that Substate::enter is manually defined or can be autogenerated for each Substate.
    fn validate_enter_defs(&mut self) -> Result<(), syn::Error> {
        for state in &mut self.states {
            let has_enter = state.imp.items.iter().any(|item| match item {
                ImplItem::Fn(fun) => fun.sig.ident == "enter",
                _ => false,
            });

            if has_enter {
                state.autogen_enter = false;
            } else if state.def_found {
                if state.has_fields {
                    return Err(syn::Error::new(
                        state.imp.trait_.as_ref().unwrap().1.span(),
                        "a struct with fields must manually implement the `Substate::enter` function",
                    ));
                } else {
                    state.autogen_enter = true;
                }
            } else {
                return Err(syn::Error::new(
                    state.imp.trait_.as_ref().unwrap().1.span(),
                    "a struct that is not defined in this module must manually implement the `Substate::enter` function",
                ));
            }
        }

        Ok(())
    }

    /// Take the machine_module if found.
    fn take_machine_mod(&mut self) -> Result<ItemMod, syn::Error> {
        match self.machine_mod.take() {
            Some(machine_mod) => Ok(machine_mod),
            None => Err(syn::Error::new(
                self.main_mod.span(),
                "no `moku::machine_module` was defined in this module",
            )),
        }
    }

    /// Take the StateMachineEvent paths if found, else ().
    /// Returns (event_from_machine, event_local) tuple.
    fn take_event(&mut self) -> (TokenStream, TokenStream) {
        match self.event.take() {
            Some(ident) => (quote! { super::#ident }, quote! { #ident }),
            None => (quote! { () }, quote! { () }),
        }
    }

    /// Get the Ident of the TopState if found.
    fn get_top_state(&self) -> Result<&Ident, syn::Error> {
        match &self.top_state {
            Some(state) => Ok(state),
            None => Err(syn::Error::new(
                self.main_mod.span(),
                "no `moku::TopState` was defined in this module",
            )),
        }
    }

    /// Validate that each Substate's superstate is another Substate or the TopState.
    fn validate_superstates(&self, top_state: &Ident) -> Result<(), syn::Error> {
        // validate that each superstate is another State or TopState
        for state in &self.states {
            let valid = state.superstate != state.ident
                && (state.superstate == *top_state
                    || self.state_idents.contains(&state.superstate));
            if !valid {
                return Err(syn::Error::new(
                    state.superstate_span,
                    format!(
                        "superstate `{}` doesn't match any other known `moku::Substate` or `moku::TopState`",
                        state.superstate
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Unpack an implementation of the `TopState` trait.
    fn unpack_top_state(&mut self, imp: ItemImpl) {
        if self.top_state.is_some() {
            self.error = Some(syn::Error::new(
                imp.span(),
                "multiple `moku::TopState`s are defined within this module",
            ));
            return;
        }

        let ident = match imp.self_ty.as_ref() {
            Type::Path(TypePath { path, .. }) => path.get_ident().cloned(),
            _ => None,
        };

        match ident {
            Some(ident) => {
                self.top_state = Some(ident);
                self.top_state_impl = Some(imp);
            }
            None => {
                self.error = Some(syn::Error::new(
                    imp.self_ty.span(),
                    "`moku::TopState` must be implemented on a plain struct \
                    \nYou may also use a type alias: `type MyTopState = Option<bool>;`",
                ));
            }
        }
    }

    /// Check if an implementation is of the `StateMachineEvent` trait and store the target type if
    /// applicable.
    fn find_event(&mut self, imp: &ItemImpl) {
        if let Some(tr) = &imp.trait_ {
            if !path_matches(&tr.1, "StateMachineEvent") {
                return;
            }
        } else {
            return;
        }

        if self.event.is_some() {
            self.error = Some(syn::Error::new(
                imp.span(),
                "multiple `moku::StateMachineEvent`s are defined within this module",
            ));
        } else {
            let ident = match imp.self_ty.as_ref() {
                Type::Path(TypePath { path, .. }) => path.get_ident().cloned(),
                _ => None,
            };

            match ident {
                Some(ident) => {
                    self.event = Some(ident);
                }
                None => {
                    self.error = Some(syn::Error::new(
                        imp.self_ty.span(),
                        "`moku::StateMachineEvent` must be implemented on a plain enum or struct. \
                        \nYou may also use a type alias: `type Event = Option<bool>;`",
                    ));
                }
            }
        }
    }

    /// Unpack an implementation of the `Substate` trait.
    fn unpack_substate(&mut self, imp: ItemImpl) {
        if !imp.generics.params.is_empty() {
            self.error = Some(syn::Error::new(
                imp.self_ty.span(),
                "`moku::Substate` impls must not have generic parameters",
            ));
            return;
        }

        let ident = match imp.self_ty.as_ref() {
            Type::Path(TypePath { path, .. }) => path.get_ident().cloned(),
            _ => None,
        };

        let ident = match ident {
            Some(ident) => ident,
            None => {
                self.error = Some(syn::Error::new(
                    imp.self_ty.span(),
                    "`moku::Substate` must be implemented on a plain struct \
                    \nYou may also use a type alias: `type MyState = Option<bool>;`",
                ));
                return;
            }
        };

        // Extract the superstate from the trait generic parameter: Substate<Parent>
        let trait_path = &imp.trait_.as_ref().unwrap().1;
        let last_segment = trait_path.segments.last().unwrap();

        let superstate = match &last_segment.arguments {
            PathArguments::AngleBracketed(args) => {
                if args.args.len() != 1 {
                    self.error = Some(syn::Error::new(
                        args.span(),
                        "`moku::Substate` requires exactly one type parameter for the superstate",
                    ));
                    return;
                }
                match args.args.first().unwrap() {
                    GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
                        path.get_ident().cloned()
                    }
                    _ => None,
                }
            }
            PathArguments::None => {
                self.error = Some(syn::Error::new(
                    last_segment.span(),
                    "`moku::Substate` requires a superstate type parameter, e.g. `impl Substate<Top> for Foo`",
                ));
                return;
            }
            PathArguments::Parenthesized(_) => {
                self.error = Some(syn::Error::new(
                    last_segment.span(),
                    "unexpected parenthesized arguments on `moku::Substate`",
                ));
                return;
            }
        };

        let superstate = match superstate {
            Some(superstate) => superstate,
            None => {
                self.error = Some(syn::Error::new(
                    last_segment.span(),
                    "the `moku::Substate` superstate must be only a single state name, e.g. `impl Substate<Top> for Foo`",
                ));
                return;
            }
        };

        let superstate_span = match &last_segment.arguments {
            PathArguments::AngleBracketed(args) => args.span(),
            _ => last_segment.span(),
        };

        // Check for duplicate Substate impls
        if !self.state_idents.insert(ident.clone()) {
            self.error = Some(syn::Error::new(
                imp.self_ty.span(),
                format!(
                    "multiple `Substate` impls found for `{}`; each state must only implement Substate once",
                    ident
                ),
            ));
            return;
        }

        self.states.push(UnpackedState {
            ident,
            superstate,
            superstate_span,
            imp,
            def_found: false,
            has_fields: false,
            autogen_enter: false,
        });
    }

    fn unpack_struct(&mut self, def: ItemStruct) -> Option<Item> {
        // track what structs have no fields for Substate::enter autogen info
        self.structs
            .insert(def.ident.clone(), !def.fields.is_empty());

        Some(Item::Struct(def))
    }

    fn unpack_mod(&mut self, module: ItemMod) -> Option<Item> {
        let mut attrs = filter_attributes(&module.attrs, "machine_module");

        match attrs.len() {
            0 => {
                // kick back modules without our attribute
                return Some(Item::Mod(module));
            }
            1 => (),
            _ => {
                self.error = Some(syn::Error::new(
                    module.span(),
                    "multiple `moku::machine_module` attributes defined for this module",
                ));
                return None;
            }
        }

        let attr = attrs.pop().unwrap();

        // validate attribute arguments
        match attr.meta {
            Meta::Path(_) => (),
            _ => {
                self.error = Some(syn::Error::new(
                    attr.span(),
                    "`moku::machine_module` accepts no arguments, try `#[moku::machine_module]`",
                ));
                return None;
            }
        }

        // validate single attribute definition in module
        if self.machine_mod.is_some() {
            self.error = Some(syn::Error::new(
                module.span(),
                "multiple `moku::machine_module`s are defined within this module",
            ));
            return None;
        }

        // validate this module has some inline content
        if let Some(content) = &module.content {
            if content.1.is_empty() {
                // all is good
                self.machine_mod = Some(module);
                return None;
            }
        }

        // fallthrough error for above validation
        let msg = format!(
            "a `moku::machine_module` must have empty braces, try `mod {} {{}}`",
            module.ident
        );
        self.error = Some(syn::Error::new(module.span(), msg));

        None
    }

    fn unpack_impl(&mut self, imp: ItemImpl) -> Option<Item> {
        let tr = match &imp.trait_ {
            None => return Some(Item::Impl(imp)),
            Some(tr) => &tr.1,
        };

        if path_matches(tr, "TopState") {
            // TopState now has no generic parameters
            self.unpack_top_state(imp);
            None
        } else if path_matches(tr, "Substate") {
            // Substate<Parent> - superstate is in the generic parameter
            self.unpack_substate(imp);
            None
        } else {
            Some(Item::Impl(imp))
        }
    }
}
