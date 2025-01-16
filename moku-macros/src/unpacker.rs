use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use quote::format_ident;
use syn::{
    spanned::Spanned, Attribute, ImplItem, Item, ItemImpl, ItemMod, ItemStruct, Meta, MetaList,
    Type, TypePath,
};

use crate::{
    metadata::{Metadata, State},
    util::{path_matches, path_matches_generic},
};

/// Collect and validate Metadata about the structure of a `state_machine` module and the usage of attributes.
pub fn build_metadata(name: Ident, module: ItemMod) -> Result<Metadata, syn::Error> {
    let mut unpacker = Unpacker::new(name, module);
    unpacker.unpack()?;
    unpacker.match_state_defs()?;
    unpacker.validate_superstates_types()?;
    unpacker.validate_enter_defs()?;
    let top_state = unpacker.get_top_state()?;
    unpacker.validate_superstates(top_state)?;
    unpacker.build_metadata()
}

fn filter_attributes<'a>(attrs: &'a [Attribute], name: &str) -> Vec<&'a Attribute> {
    attrs
        .iter()
        .filter(move |attr| path_matches(attr.meta.path(), name))
        .collect()
}

struct UnpackedState {
    ident: Ident,
    superstate: Ident,
    imp: ItemImpl,
    attr_span: Span,
    can_autogen_enter: bool,
    autogen_enter: bool,
}

struct Unpacker {
    name: Ident,
    main_mod: ItemMod,
    machine_mod: Option<ItemMod>,
    top_state: Option<Ident>,
    states: Vec<UnpackedState>,
    structs: HashMap<Ident, bool>,
    error: Option<syn::Error>,
}

impl Unpacker {
    fn new(name: Ident, main_mod: ItemMod) -> Self {
        Self {
            name,
            main_mod,
            machine_mod: None,
            top_state: None,
            states: Vec::new(),
            structs: HashMap::new(),
            error: None,
        }
    }

    /// Build Metadata from the info collected by an Unpacker.
    fn build_metadata(mut self) -> Result<Metadata, syn::Error> {
        let mut metadata = Metadata {
            top_state: self.get_top_state()?.into(),
            machine_mod: self.take_machine_mod()?,
            state_enum: format_ident!("{}State", self.name),
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
            "a `moku::state_machine` module must be inline with its attribte, try
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

    /// Match each State with it's struct definition.
    fn match_state_defs(&mut self) -> Result<(), syn::Error> {
        for state in &mut self.states {
            match self.structs.remove(&state.ident) {
                Some(has_no_fields) => state.can_autogen_enter = has_no_fields,
                None => {
                    return Err(syn::Error::new(
                        state.imp.self_ty.span(),
                        format!("could not find the struct definition for this State in this module (or this is a duplicate impl of State for {})", state.ident),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Validate that the Superstates associated type is not manually defined for any State.
    fn validate_superstates_types(&self) -> Result<(), syn::Error> {
        for state in &self.states {
            for item in &state.imp.items {
                if let ImplItem::Type(ty) = item {
                    if ty.ident == "Superstates" {
                        return Err(syn::Error::new(
                            ty.span(),
                            "the `State::Superstates` associated type must not be manually defined",
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate that State::enter is manually defined or can be autogenerated for each State.
    fn validate_enter_defs(&mut self) -> Result<(), syn::Error> {
        for state in &mut self.states {
            let has_enter = state.imp.items.iter().any(|item| match item {
                ImplItem::Fn(fun) => fun.sig.ident == "enter",
                _ => false,
            });

            if has_enter {
                state.autogen_enter = false;
            } else if state.can_autogen_enter {
                state.autogen_enter = true;
            } else {
                return Err(syn::Error::new(
                    state.imp.trait_.as_ref().unwrap().1.span(),
                    "a struct with fields must manually implement the `State::enter` function",
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

    /// Validate that each State's superstate is another State or the TopState.
    fn validate_superstates(&self, top_state: &Ident) -> Result<(), syn::Error> {
        // validate that each superstate is another State or TopState
        for (index, state) in self.states.iter().enumerate() {
            let matches_top_state = state.superstate == *top_state;
            let matches_other_state =
                self.states
                    .iter()
                    .enumerate()
                    .any(|(other_index, other_state)| {
                        index != other_index && state.superstate == other_state.ident
                    });
            if !matches_top_state && !matches_other_state {
                return Err(syn::Error::new(
                    state.attr_span,
                    format!(
                        "superstate `{}` doesn't match any other known `moku::State` or `moku::TopState`",
                        state.superstate
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Unpack an implementation of the `TopState` trait.
    fn unpack_top_state(&mut self, imp: &ItemImpl) {
        if self.top_state.is_some() {
            self.error = Some(syn::Error::new(
                imp.span(),
                "multiple `moku::TopState`s are defined within this module",
            ));
        } else {
            let ident = match imp.self_ty.as_ref() {
                Type::Path(TypePath { path, .. }) => path.get_ident().cloned(),
                _ => None,
            };

            match ident {
                Some(ident) => {
                    self.top_state = Some(ident);
                }
                None => {
                    self.error = Some(syn::Error::new(
                        imp.self_ty.span(),
                        "`moku::TopState` must be implemented on a plain struct",
                    ));
                }
            }
        }
    }

    /// Unpack an implementation of the `State` trait.
    fn unpack_state(&mut self, imp: ItemImpl) {
        if !imp.generics.params.is_empty() {
            self.error = Some(syn::Error::new(
                imp.self_ty.span(),
                "`moku::State`s must not have generic parameters",
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
                    "`moku::State` must be implemented on a plain struct",
                ));
                return;
            }
        };

        let mut attrs = filter_attributes(&imp.attrs, "superstate");
        match attrs.len() {
            0 => {
                self.error = Some(syn::Error::new(
                    imp.span(),
                    "no `moku::superstate` attribute defined for this `moku::State`",
                ));
                return;
            }
            1 => (),
            _ => {
                self.error = Some(syn::Error::new(
                    imp.span(),
                    "multiple `moku::superstate` attributes defined for this `moku::State`",
                ));
                return;
            }
        }

        let attr = &attrs.pop().unwrap();

        let superstate: Option<Ident> = match &attr.meta {
            Meta::List(MetaList { tokens, .. }) => syn::parse2(tokens.clone()).ok(),
            _ => None,
        };

        let superstate = match superstate {
            Some(superstate) => superstate,
            None => {
                self.error = Some(syn::Error::new( imp.span(), "the `moku::superstate` attribute requires a single State name as an argument, e.g. `#[superstate(Top)]`",));
                return;
            }
        };

        self.states.push(UnpackedState {
            ident,
            superstate,
            attr_span: attr.span(),
            imp,
            can_autogen_enter: false,
            autogen_enter: false,
        });
    }

    fn unpack_struct(&mut self, def: ItemStruct) -> Option<Item> {
        // track what structs have fields for State::enter autogen info
        self.structs
            .insert(def.ident.clone(), def.fields.is_empty());
        Some(Item::Struct(def))
    }

    fn unpack_mod(&mut self, module: ItemMod) -> Option<Item> {
        // short circuit if we've found an issue
        if self.error.is_some() {
            return None;
        };

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
                    "`moku::machine_module` accepts no arguments, try `#[machine_module]`",
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
        // short circuit if we've found an issue
        if self.error.is_some() {
            return None;
        };

        let tr = match &imp.trait_ {
            None => return None,
            Some(tr) => &tr.1,
        };

        let state_enum = self.name.to_string() + "State";

        if path_matches_generic(tr, "TopState", Some(&state_enum)) {
            self.unpack_top_state(&imp);
            return Some(Item::Impl(imp));
        } else if path_matches_generic(tr, "State", Some(&state_enum)) {
            self.unpack_state(imp)
        } else if path_matches_generic(tr, "TopState", None) {
            let msg =
                format!("implementations of `moku::TopState` in this module must use only `{state_enum}` as the generic");
            self.error = Some(syn::Error::new(tr.span(), msg));
        } else if path_matches_generic(tr, "State", None) {
            let msg =
                format!("implementations of `moku::State` in this module must use only `{state_enum}` as the generic");
            self.error = Some(syn::Error::new(tr.span(), msg));
        }

        None
    }
}
