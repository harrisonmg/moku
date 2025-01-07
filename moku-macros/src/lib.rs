#![allow(unused)]

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{
    parse::Parse, parse_macro_input, spanned::Spanned, visit::Visit, Attribute, ItemImpl, ItemMod,
    ItemStruct, Meta, Path,
};

#[proc_macro_attribute]
pub fn machine_module(_args: TokenStream, input: TokenStream) -> TokenStream {
    // validate that this attribute is attached to a module
    let input_clone = input.clone();
    parse_macro_input!(input_clone as ItemMod);
    input
}

#[proc_macro_attribute]
pub fn superstate(_args: TokenStream, input: TokenStream) -> TokenStream {
    // validate that this attribute is attached to an impl
    let input_clone = input.clone();
    let imp = parse_macro_input!(input_clone as ItemImpl);

    // validate that the impl is for the State trait
    if imp
        .trait_
        .as_ref()
        .map_or(true, |tr| !path_matches_any_generic(&tr.1, "State"))
    {
        syn::Error::new(
            imp.span(),
            "`moku::superstate` must only be applied to implementations of the `moku::State` trait",
        )
        .to_compile_error()
        .into()
    } else {
        input
    }
}

#[proc_macro_attribute]
pub fn state_machine(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemMod);

    let name = if args.is_empty() {
        // derive state machine name from module name by default
        Ident::new(
            &input.ident.to_string().to_case(Case::UpperCamel),
            Span::call_site(),
        )
    } else {
        // name is specificed in attribute args
        parse_macro_input!(args as Ident)
    };

    match generate_state_machine(name, input) {
        Err(error) => error.into_compile_error().into(),
        Ok(output) => output.into_token_stream().into(),
    }
}

fn generate_state_machine(name: Ident, input: ItemMod) -> Result<ItemMod, syn::Error> {
    let mut visitor = Visitor::new(name);
    let metadata = visitor.get_metadata(&input)?;
    Ok(input)
}

/// Check if a Path matches `name` or `moku::{name}`.
fn path_matches(path: &Path, name: &str) -> bool {
    let qualified_name = format!("moku::{name}");
    path.is_ident(name) || path.is_ident(&qualified_name)
}

/// Check that a Path matches `{name}<{generic}>` or `moku::{name}<{generic}>`
fn path_matches_generic(path: &Path, name: &str, generic: &str) -> bool {
    todo!()
}

/// Check that a Path matches `{name}<{any generic}>` or `moku::{name}<{any generic}>`
fn path_matches_any_generic(path: &Path, name: &str) -> bool {
    todo!()
}

/// Filter Attributes based on their Path matching `name` or `moku::{name}`.
fn filter_attributes<'a>(
    attrs: &'a Vec<Attribute>,
    name: &'a str,
) -> impl Iterator<Item = &'a Attribute> + 'a {
    attrs
        .iter()
        .filter(move |attr| path_matches(&attr.meta.path(), name))
}

struct Visitor<'ast> {
    name: Ident,
    machine_module: Option<&'ast ItemMod>,
    top_state: Option<Ident>,
    states: Vec<(Ident, &'ast ItemImpl, Ident)>,
    error: Option<syn::Error>,
}

impl<'ast> Visitor<'ast> {
    fn new(name: Ident) -> Self {
        Self {
            name,
            machine_module: None,
            top_state: None,
            states: Vec::new(),
            error: None,
        }
    }

    /// Collect and validate Metadata about the structure of a `state_machine` module and the usage of attributes.
    fn get_metadata(mut self, module: &'ast ItemMod) -> Result<Metadata, syn::Error> {
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

        // TODO validate that each superstate is another State or TopState

        todo!()
    }

    fn visit_top_state(&mut self, imp: &'ast ItemImpl) {}

    fn visit_state(&mut self, imp: &'ast ItemImpl) {}
}

impl<'ast> Visit<'ast> for Visitor<'ast> {
    /// Search for the machine module and validate that exactly one is defined.
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

    /// Search for states and validate that exactly one TopState is defined.
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

        if path_matches_generic(tr, "TopState", &state_enum) {
            self.visit_top_state(imp);
        } else if path_matches_generic(tr, "State", &state_enum) {
            self.visit_state(imp);
        } else if path_matches_any_generic(tr, "TopState") {
            let msg =
                format!("implementations of moku::TopState in this module must use {state_enum} as the generic");
            self.error = Some(syn::Error::new(imp.span(), msg));
        } else if path_matches_any_generic(tr, "State") {
            let msg =
                format!("implementations of moku::State in this module must use {state_enum} as the generic");
            self.error = Some(syn::Error::new(imp.span(), msg));
        }

        // TODO validate that the state impls use the correct StateEnum
        // TODO validate that no state is missing a superstate attr

        // TODO validate that there is only one top state
        // TODO validate that Superstates is not defined
        // TODO validate that there is one superstate arg
    }
}

struct State {
    ident: Ident,
    node: Ident,
    substate_enum: Ident,
    superstates_enum: Ident,
    ancestors: Vec<State>,
    children: Vec<State>,
}

struct Metadata {
    states: Vec<State>,
    top_state: State,
}
