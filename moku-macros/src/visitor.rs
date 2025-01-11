use std::collections::HashMap;

use proc_macro2::Ident;
use syn::{
    spanned::Spanned, Attribute, ImplItem, Item, ItemImpl, ItemMod, ItemStruct, Meta, MetaList,
    Type, TypePath,
};

use crate::{
    metadata::Metadata,
    util::{path_matches, path_matches_generic},
};

/// Collect and validate Metadata about the structure of a `state_machine` module and the usage of attributes.
pub fn build_metadata(name: Ident, module: &ItemMod) -> Result<Metadata, syn::Error> {
    let mut visitor = Visitor::new(name, module);
    visitor.visit()?;
    visitor.match_state_defs()?;
    visitor.validate_superstates_types()?;
    visitor.validate_enter_defs()?;
    let top_state = visitor.get_top_state()?;
    visitor.validate_superstates(top_state)?;
    visitor.build_metadata()
}

fn filter_attributes<'a>(
    attrs: &'a [Attribute],
    name: &'a str,
) -> impl Iterator<Item = &'a Attribute> + 'a {
    attrs
        .iter()
        .filter(move |attr| path_matches(attr.meta.path(), name))
}

struct VisitedState<'ast> {
    ident: Ident,
    superstate: Ident,
    imp: &'ast ItemImpl,
    attr: &'ast Attribute,
    def: Option<&'ast ItemStruct>,
    autogen_enter: bool,
}

struct Visitor<'ast> {
    name: Ident,
    module: &'ast ItemMod,
    machine_mod: Option<Ident>,
    top_state: Option<Ident>,
    states: Vec<VisitedState<'ast>>,
    structs: HashMap<Ident, &'ast ItemStruct>,
    error: Option<syn::Error>,
}

impl<'ast> Visitor<'ast> {
    fn new(name: Ident, module: &'ast ItemMod) -> Self {
        Self {
            name,
            module,
            machine_mod: None,
            top_state: None,
            states: Vec::new(),
            structs: HashMap::new(),
            error: None,
        }
    }

    /// Build Metadata from the info collected by a Visitor.
    fn build_metadata(self) -> Result<Metadata, syn::Error> {
        let mut metadata = Metadata {
            top_state: self.get_top_state()?.into(),
            machine_mod: self.get_machine_mod()?.clone(),
            name: self.name,
            states: HashMap::new(),
        };

        for state in &self.states {
            metadata.add_state(&state.ident, state.autogen_enter);
        }

        for state in &self.states {
            metadata.add_relation(&state.superstate, &state.ident)?;
        }

        Ok(metadata)
    }

    /// Visit each item in the module's content.
    fn visit(&mut self) -> Result<(), syn::Error> {
        if let Some(content) = &self.module.content {
            for item in &content.1 {
                match item {
                    Item::Struct(item) => self.visit_item_struct(item),
                    Item::Mod(item) => self.visit_item_mod(item),
                    Item::Impl(item) => self.visit_item_impl(item),
                    _ => (),
                }

                // stop if we encounter an issue
                if let Some(error) = self.error.take() {
                    return Err(error);
                }
            }
        }

        Ok(())
    }

    /// Match each State with it's struct definition.
    fn match_state_defs(&mut self) -> Result<(), syn::Error> {
        for state in &mut self.states {
            state.def = self.structs.remove(&state.ident);
            if state.def.is_none() {
                return Err(syn::Error::new(
                    state.imp.self_ty.span(),
                    format!("could not find the struct definition for this State in this module (or this is a duplicate impl of State for {})", state.ident),
                ));
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
            let def = state.def.unwrap();
            let can_autogen = def.fields.is_empty();
            let has_enter = state.imp.items.iter().any(|item| match item {
                ImplItem::Fn(fun) => fun.sig.ident == "enter",
                _ => false,
            });

            if has_enter {
                state.autogen_enter = false;
            } else if can_autogen {
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

    /// Get a reference to the machine_module if found.
    fn get_machine_mod(&self) -> Result<&Ident, syn::Error> {
        match &self.machine_mod {
            Some(machine_mod) => Ok(machine_mod),
            None => Err(syn::Error::new(
                self.module.span(),
                "no `moku::machine_module` was defined in this module",
            )),
        }
    }

    /// Get the Ident of the TopState if found.
    fn get_top_state(&self) -> Result<&Ident, syn::Error> {
        match &self.top_state {
            Some(state) => Ok(state),
            None => Err(syn::Error::new(
                self.module.span(),
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
                    state.attr.span(),
                    format!(
                        "superstate `{}` doesn't match any other known `moku::State` or `moku::TopState`",
                        state.superstate
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Visit an implementation of the `TopState` trait.
    fn visit_top_state(&mut self, imp: &'ast ItemImpl) {
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

    /// Visit an implementation of the `State` trait.
    fn visit_state(&mut self, imp: &'ast ItemImpl) {
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

        let mut attrs: Vec<_> = filter_attributes(&imp.attrs, "superstate").collect();
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
                self.error = Some(syn::Error::new(
            imp.span(),
            "the `moku::superstate` attribute requires a single State name as an argument, e.g. `#[superstate(Top)]`",
        ));
                return;
            }
        };

        self.states.push(VisitedState {
            ident,
            superstate,
            imp,
            attr,
            def: None,
            autogen_enter: false,
        });
    }

    fn visit_item_struct(&mut self, def: &'ast ItemStruct) {
        // hold onto these for matching with States later
        self.structs.insert(def.ident.clone(), def);
    }

    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        // short circuit if we've found an issue
        if self.error.is_some() {
            return;
        };

        for attr in filter_attributes(&module.attrs, "machine_module") {
            // validate attribute arguments
            match attr.meta {
                Meta::Path(_) => (),
                _ => {
                    self.error = Some(syn::Error::new(
                        attr.span(),
                        "`moku::machine_module` accepts no arguments, try `#[machine_module]`",
                    ));
                    return;
                }
            }

            // validate single attribute definition in module
            if self.machine_mod.is_some() {
                self.error = Some(syn::Error::new(
                    module.span(),
                    "multiple `moku::machine_module`s are defined within this module",
                ));
                return;
            }

            // validate this module has some inline content
            if let Some(content) = &module.content {
                if content.1.is_empty() {
                    // all is good
                    self.machine_mod = Some(module.ident.clone());
                    return;
                }
            }

            // fallthrough error for above validation
            let msg = format!(
                "a `moku::machine_module` must have empty braces, try `mod {} {{}}`",
                module.ident
            );
            self.error = Some(syn::Error::new(module.span(), msg))
        }
    }

    fn visit_item_impl(&mut self, imp: &'ast ItemImpl) {
        // short circuit if we've found an issue
        if self.error.is_some() {
            return;
        };

        let tr = match &imp.trait_ {
            None => return,
            Some(tr) => &tr.1,
        };

        let state_enum = self.name.to_string() + "State";

        if path_matches_generic(tr, "TopState", Some(&state_enum)) {
            self.visit_top_state(imp);
        } else if path_matches_generic(tr, "State", Some(&state_enum)) {
            self.visit_state(imp);
        } else if path_matches_generic(tr, "TopState", None) {
            let msg =
                format!("implementations of `moku::TopState` in this module must use only `{state_enum}` as the generic");
            self.error = Some(syn::Error::new(imp.trait_.as_ref().unwrap().1.span(), msg));
        } else if path_matches_generic(tr, "State", None) {
            let msg =
                format!("implementations of `moku::State` in this module must use only `{state_enum}` as the generic");
            self.error = Some(syn::Error::new(imp.trait_.as_ref().unwrap().1.span(), msg));
        }
    }
}
