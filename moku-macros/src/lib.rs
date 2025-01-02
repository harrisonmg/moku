#![allow(unused)]

use core::panic;

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
    let module = parse_macro_input!(input_clone as ItemMod);
    input
}

#[proc_macro_attribute]
pub fn superstate(_args: TokenStream, input: TokenStream) -> TokenStream {
    // validate that this attribute is attached to an impl
    let input_clone = input.clone();
    parse_macro_input!(input_clone as ItemImpl);
    input
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

// TODO remove extra-traits syn feature

fn generate_state_machine(name: Ident, input: ItemMod) -> Result<ItemMod, syn::Error> {
    let mut visitor = Visitor::default();
    visitor.visit(&input)?;
    Ok(input)
}

#[derive(Default)]
struct Visitor<'ast> {
    module: Option<&'ast ItemMod>,
    top_state: Option<&'ast ItemImpl>,
    states: Vec<&'ast ItemImpl>,
    error: Option<syn::Error>,
}

impl<'ast> Visitor<'ast> {
    fn visit(&mut self, module: &'ast ItemMod) -> Result<(), syn::Error> {
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
                "a moku `state_machine` module must have inline content",
            ));
        }

        Ok(())
    }
}

/// Filter Attributes based on their Path matching `name` or `moku::{name}`.
fn filter_attributes<'a>(
    attrs: &'a Vec<Attribute>,
    name: &'a str,
) -> impl Iterator<Item = &'a Attribute> + 'a {
    let qualified_name = format!("moku::{name}");
    attrs.iter().filter(move |attr| {
        let path = attr.meta.path();
        path.is_ident(name) || path.is_ident(&qualified_name)
    })
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
                        "moku `machine_module` accepts no arguments, try `#[machine_module]`",
                    ));
                    return;
                }
            }

            // validate single attribute definition in module
            if self.module.is_some() {
                self.error = Some(syn::Error::new(
                    module.span(),
                    "multiple moku `machine_module`s are defined within this module",
                ));
                return;
            }

            // validate this module has some inline content
            if let Some(content) = &module.content {
                if content.1.is_empty() {
                    // all is good
                    self.module = Some(module);
                    return;
                }
            }

            // fallthrough error for above validation
            let msg = format!(
                "a moku machine_module must have empty braces, try `mod {} {{}}`",
                module.ident
            );
            self.error = Some(syn::Error::new(module.span(), msg))
        }
    }

    /// Search for states and validate that exactly one TopState is defined.
    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        // short circuit if we've found an issue
        if self.error.is_some() {
            return;
        };

        // TODO start here
    }
}

struct State {
    ident: Ident,
    node: Ident,
    substate_enum: Ident,
    superstates_enum: Ident,
    children: Vec<State>,
}

struct Metadata {
    states: Vec<State>,
}
