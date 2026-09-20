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
pub struct WhiteSpace;
impl LexRule<CargryTokens> for WhiteSpace {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
        let white_space = lreduce(
            |s1, s2| s1 + s2,
            lfmany0(lor(
                lstring(" ", lign()),
                lor(lstring("\n", lign()), lstring("\t", lign())),
            )),
        );
        white_space(input)
    }
}

#[lexer(CargryTokens)]
pub struct EndLine;
impl LexRule<CargryTokens> for EndLine {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
        let f = lstring(";", lign());
        f(input)
    }
}
