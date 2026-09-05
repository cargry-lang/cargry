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

use crate::lexer::CargryTokens;

#[lexer(CargryTokens)]
pub struct Struct;
impl LexRule<CargryTokens> for Struct {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
        let f = lstring("struct", lign());
        f(input)
    }
}

#[lexer(CargryTokens)]
pub struct Fun;
impl LexRule<CargryTokens> for Fun {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
        let f = lstring("fun", lign());
        f(input)
    }
}

#[lexer(CargryTokens)]
pub struct Let;
impl LexRule<CargryTokens> for Let {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
        let f = lstring("let", lign());
        f(input)
    }
}
