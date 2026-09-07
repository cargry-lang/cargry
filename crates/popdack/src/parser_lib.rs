/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lewekk::Lexer;

use crate::parser_utils::{PNode, PStatus};

/// sample:
/// ```
/// use lewekk::Lexer;
/// use lewekk_head::LexRule;
/// use lewekk_utils::{lign, lstring};
///
/// #[lewer(Tokens)]
/// pub struct A;
/// impl LexRule<Tokens> for A {
///     fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
///         let f = lstring("a", lign());
///         f(input)
///     }
/// }
///
/// #[tokens(A)]
/// enum Tokens;
/// enum Nodes {
///     A(Option<Box<Nodes>>),
/// }
///
/// let mut l = Lexer::<Tokens>::new(None);
/// l.add_rule(A);
/// let p = Parser::<Tokens, Nodes>::new(l);
/// ```
pub struct Parser<Tokens, Nodes> {
    lexer: Lexer<Tokens>,
    node: Option<Nodes>,
}
impl<Tokens, Nodes> Parser<Tokens, Nodes> {
    pub fn new(lexer: Lexer<Tokens>) -> Self {
        Self { lexer, node: None }
    }
    pub fn run(&mut self) {}
}
