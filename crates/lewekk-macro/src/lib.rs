/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::clone::Clone;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    DeriveInput, Error, Ident, Item, Token, parse::Parse, parse_macro_input, punctuated::Punctuated,
};

#[proc_macro_attribute]
pub fn lexer(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let cloned_input = input.clone();
    let name = cloned_input.ident;
    let name_str = name.to_string();

    let tokens = parse_macro_input!(attr with Ident::parse);

    let lex_clone_impl = quote! {
        #[derive(Clone)]
        #input

        impl LexMapping<#tokens> for #name {
            fn ltoken(&self) -> #tokens {
                #tokens::#name
            }
        }

        impl LexClone<#tokens> for #name {
            fn clone_box(&self) -> Box<dyn LexRule<#tokens>> {
                Box::new(Clone::clone(self))
            }
        }

        impl LexDisplay for #name {
            fn get_name(&self) -> &'static str {
                #name_str
            }
        }
    };

    TokenStream::from(lex_clone_impl)
}

#[proc_macro_attribute]
pub fn tokens(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let input_enum = match input {
        Item::Enum(item_enum) => item_enum,
        _ => {
            return Error::new_spanned(input, "This attribute macro can apply only enum.")
                .to_compile_error()
                .into();
        }
    };

    let cloned_input = input_enum.clone();
    let vis = cloned_input.vis;
    let name = cloned_input.ident;

    let args = parse_macro_input!(attr with Punctuated::<Ident, Token![,]>::parse_terminated);
    let yield_args = args.iter().map(|ident| quote! {#ident,});
    let variants = args.iter().map(|ident| {
        let fn_token = ident.to_string().to_lowercase();
        let fn_ident = format_ident!("token_{}", fn_token, span = ident.span());
        quote! {
            #[inline]
            pub fn #fn_ident() -> #name {
                #name::#ident
            }
        }
    });

    let lex_tokens_impl = quote! {
        #[derive(Clone, Debug, Eq, PartialEq)]
        #vis enum #name {
            #(#yield_args)*
        }

        #(#variants)*
    };

    TokenStream::from(lex_tokens_impl)
}
