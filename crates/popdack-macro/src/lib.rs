/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, ItemFn, parse::Parse, parse_macro_input};

#[proc_macro_attribute]
pub fn token_eater(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let token = parse_macro_input!(attr with Ident::parse);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_body = &input_fn.block;

    let chain = fn_body.stmts.iter().map(|stmt| {
        let v: Vec<String> = stmt
            .to_token_stream()
            .to_string()
            .split(' ')
            .map(|x| x.to_string())
            .collect();
        let v = v.iter().map(|item| match item {
            item if item.starts_with("@") => {
                let v_ident = item[1..].to_string();
                let ident = format_ident!("{}", v_ident);
                quote! { .token(#token::#ident) }
            }
            item if item.starts_with("<") && item.ends_with(">") => {
                let v_ident = item[1..(item.len() - 1)].to_string();
                let ident = format_ident!("{}", v_ident);
                quote! { .call_f(#ident) }
            }
            _ => quote! {},
        });
        quote! { #(#v)* }
    });

    TokenStream::from(quote! {
        #fn_vis fn #fn_name<'a>(#fn_inputs) -> PdkParser<'a, #token> {
            pinit()#(#chain)*
        }
    })
}
