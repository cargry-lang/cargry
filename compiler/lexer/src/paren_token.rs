/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::result;

use lewekk_head::*;
use lewekk_macro::*;
use lewekk_utils::*;

use crate::lexer::CargryTokens;

#[lexer(CargryTokens)]
pub struct LParen;
impl LexRule<CargryTokens> for LParen {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
        let f = lstring("(", lign());
        f(input)
    }
}

#[lexer(CargryTokens)]
pub struct RParen;
impl LexRule<CargryTokens> for RParen {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
        let f = lstring(")", lign());
        f(input)
    }
}
