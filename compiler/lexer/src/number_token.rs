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
pub struct Number;
impl LexRule<CargryTokens> for Number {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
        let f = lreduce(
            |s1, s2| s1 + s2,
            lfmany1(lpredicate(
                |c| c.is_numeric(),
                lor(
                    lstring(
                        ".",
                        lreduce(
                            |s1, s2| s1 + s2,
                            lfmany1(lpredicate(|c| c.is_numeric(), lign())),
                        ),
                    ),
                    lign(),
                ),
            )),
        );
        f(input.as_str())
    }
}
