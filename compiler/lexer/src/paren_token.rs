/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lewekk_head::*;
use lewekk_macro::*;
use lewekk_utils::*;

#[lexer]
pub struct LParen;
impl LexRule for LParen {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lstring("(", lign());
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}

#[lexer]
pub struct RParen;
impl LexRule for RParen {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lstring(")", lign());
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}
