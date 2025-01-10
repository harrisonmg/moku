#![allow(unused)]

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{parse_macro_input, spanned::Spanned, ItemImpl, ItemMod};
use util::path_matches_generic;
use visitor::Visitor;

mod metadata;
mod util;
mod visitor;

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
        .map_or(true, |tr| !path_matches_generic(&tr.1, "State", None))
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
    let metadata = Visitor::get_metadata(name, &input)?;
    // TODO use metadata to generate state machine
    Ok(input)
}
