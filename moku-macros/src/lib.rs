#![allow(unused)]

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{parse, parse_macro_input, spanned::Spanned, ItemImpl, ItemMod};
use unpacker::build_metadata;
use util::path_matches_generic;

mod metadata;
mod unpacker;
mod util;

/// Append a compile error to a TokenStream.
///
/// Important for allowing rust-analyzer completions to work while typing inside of an attribute
/// macro, which would otherwise erase the attributed item during macro expansion in favor of the
/// compile error resulting from trying to parse a malformed AST.
///
/// See https://github.com/tokio-rs/tokio/pull/4162 for more context.
fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

#[proc_macro_attribute]
pub fn machine_module(_args: TokenStream, input: TokenStream) -> TokenStream {
    // validate that this attribute is attached to a module
    match parse::<ItemMod>(input.clone()) {
        Ok(_) => input,
        Err(error) => token_stream_with_error(input, error),
    }
}

#[proc_macro_attribute]
pub fn superstate(_args: TokenStream, input: TokenStream) -> TokenStream {
    // validate that this attribute is attached to an impl
    let imp = match parse::<ItemImpl>(input.clone()) {
        Ok(imp) => imp,
        Err(error) => {
            return token_stream_with_error(input, error);
        }
    };

    // validate that the impl is for the State trait
    if imp
        .trait_
        .as_ref()
        .map_or(false, |tr| path_matches_generic(&tr.1, "State", None))
    {
        input
    } else {
        token_stream_with_error(
            input,
            syn::Error::new(
                imp.span(),
                "`moku::superstate` must only be applied to implementations of the `moku::State` trait",
            )
        )
    }
}

#[proc_macro_attribute]
pub fn state_machine(args: TokenStream, input: TokenStream) -> TokenStream {
    // validate that this attribute is attached to a module
    let main_mod = match parse::<ItemMod>(input.clone()) {
        Ok(main_mod) => main_mod,
        Err(error) => {
            return token_stream_with_error(input, error);
        }
    };

    // Past this point, don't return the input along with the compile error here.
    //
    // If we've found an error at this point, the lack of autogen code will cause a ton of
    // red herring compile errors within the machine module. Better to let the errors be
    // outside of the module due to its lack of existence in order to make the true error
    // easier for the user to find within the module.
    //
    // This will stop rust-analyzer completions from working while the error persists.

    let name = if args.is_empty() {
        // derive state machine name from module name by default
        Ident::new(
            &main_mod.ident.to_string().to_case(Case::UpperCamel),
            Span::call_site(),
        )
    } else {
        parse_macro_input!(args as Ident)
    };

    match generate_state_machine(name, main_mod) {
        Ok(output) => output.into_token_stream().into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn generate_state_machine(name: Ident, main_mod: ItemMod) -> Result<ItemMod, syn::Error> {
    let metadata = build_metadata(name, main_mod)?;
    Ok(metadata.write_state_machine())
}
