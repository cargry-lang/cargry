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
pub struct Number;
impl LexRule for Number {
    fn lparse(&mut self, input: &mut String) -> LexResult {
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
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}
