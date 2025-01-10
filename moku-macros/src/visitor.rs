use proc_macro2::Ident;
use syn::{
    spanned::Spanned, visit::Visit, Attribute, ItemImpl, ItemMod, ItemStruct, Meta, MetaList, Type,
    TypePath,
};

use crate::{
    metadata::Metadata,
    util::{path_matches, path_matches_generic},
};

/// Filter Attributes based on their Path matching `name` or `moku::{name}`.
fn filter_attributes<'a>(
    attrs: &'a Vec<Attribute>,
    name: &'a str,
) -> impl Iterator<Item = &'a Attribute> + 'a {
    attrs
        .iter()
        .filter(move |attr| path_matches(&attr.meta.path(), name))
}

struct VisitedState<'ast> {
    ident: Ident,
    superstate: Ident,
    imp: &'ast ItemImpl,
    attr: &'ast Attribute,
    def: Option<&'ast ItemStruct>,
}

pub struct Visitor<'ast> {
    name: Ident,
    machine_module: Option<&'ast ItemMod>,
    top_state: Option<Ident>,
    states: Vec<VisitedState<'ast>>,
    error: Option<syn::Error>,
}

impl<'ast> Visitor<'ast> {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            machine_module: None,
            top_state: None,
            states: Vec::new(),
            error: None,
        }
    }

    /// Collect and validate Metadata about the structure of a `state_machine` module and the usage of attributes.
    pub fn get_metadata(mut self, module: &'ast ItemMod) -> Result<Metadata, syn::Error> {
        if let Some(content) = &module.content {
            // visit each item separately so that we can override `visit_item_mod`
            for item in &content.1 {
                self.visit_item(item);

                // stop if we encounter an issue
                if let Some(error) = self.error.take() {
                    return Err(error);
                }
            }
        } else {
            return Err(syn::Error::new(
                module.span(),
                "a `moku::state_machine` module must have inline content",
            ));
        }

        self.find_state_defs(module)?;

        let machine_module = match self.machine_module {
            Some(module) => module,
            None => {
                return Err(syn::Error::new(
                    module.span(),
                    "no `moku::machine_module` was defined in this module",
                ))
            }
        };

        let top_state = match self.top_state {
            Some(state) => state,
            None => {
                return Err(syn::Error::new(
                    module.span(),
                    "no `moku::TopState` was defined in this module",
                ))
            }
        };

        // validate that each superstate is another State or TopState
        for (index, state) in self.states.iter().enumerate() {
            let matches_top_state = state.superstate == top_state;
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
                        "superstate `{}` doesn't match any known `moku::State` or `moku::TopState`",
                        state.superstate
                    ),
                ));
            }
        }

        // TODO create metadata
        todo!()
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
                Type::Path(TypePath { path, .. }) => path.get_ident().map(Clone::clone),
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
            Type::Path(TypePath { path, .. }) => path.get_ident().map(Clone::clone),
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
        });
    }

    fn find_state_defs(&mut self, module: &'ast ItemMod) -> Result<(), syn::Error> {
        for state in &self.states {}

        // TODO check that we've found the definition for each state
        Ok(())
    }
}

impl<'ast> Visit<'ast> for Visitor<'ast> {
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
            if self.machine_module.is_some() {
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
                    self.machine_module = Some(module);
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
