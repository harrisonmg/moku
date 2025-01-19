#![allow(unused)]

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{parse_macro_input, spanned::Spanned, ItemImpl, ItemMod};
use unpacker::build_metadata;
use util::path_matches_generic;

mod metadata;
mod unpacker;
mod util;

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
        .map_or(false, |tr| path_matches_generic(&tr.1, "State", None))
    {
        input
    } else {
        syn::Error::new(
            imp.span(),
            "`moku::superstate` must only be applied to implementations of the `moku::State` trait",
        )
        .to_compile_error()
        .into()
    }
}

#[proc_macro_attribute]
pub fn state_machine(args: TokenStream, input: TokenStream) -> TokenStream {
    let main_mod = parse_macro_input!(input as ItemMod);

    let name = if args.is_empty() {
        // derive state machine name from module name by default
        Ident::new(
            &main_mod.ident.to_string().to_case(Case::UpperCamel),
            Span::call_site(),
        )
    } else {
        // name is specificed in attribute args
        parse_macro_input!(args as Ident)
    };

    match generate_state_machine(name, main_mod) {
        Err(error) => error.into_compile_error().into(),
        Ok(output) => output.into_token_stream().into(),
    }
}

fn generate_state_machine(name: Ident, main_mod: ItemMod) -> Result<ItemMod, syn::Error> {
    let metadata = build_metadata(name, main_mod)?;
    Ok(metadata.write_state_machine())
}
