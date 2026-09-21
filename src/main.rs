/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use popdack_macro::token_eater;

use lexer::lexer::CargryTokens;
use popdack::pinit;

#[token_eater]
fn g() {}

#[token_eater]
fn f() {
    <g>
}

fn main() {}
