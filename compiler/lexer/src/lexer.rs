/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lewekk::Lexer;
use lewekk_head::LexerManager;
use lewekk_macro::tokens;

use crate::define_token::{Fun, Let, Struct};
use crate::endline_token::{EndLine, WhiteSpace};
use crate::ident_token::Ident;
use crate::module_token::{Mod, Use};
use crate::number_token::Number;
use crate::operator_token::EqOp;
use crate::paren_token::{LParen, RParen};
use crate::scope_token::{LScope, RScope};

#[tokens(
    WhiteSpace, EndLine, Ident, LScope, RScope, LParen, RParen, Number, Use, Mod, Struct, Fun, Let,
    EqOp
)]
pub enum CargryTokens {}

pub struct CargryLexer {
    lexer: Lexer<CargryTokens>,
}
impl LexerManager<CargryTokens> for CargryLexer {
    fn new() -> Self {
        let mut lex = Lexer::new(Some(WhiteSpace));
        lex.add_rule(EndLine);
        lex.add_rule(LScope);
        lex.add_rule(RScope);
        lex.add_rule(LParen);
        lex.add_rule(RParen);
        lex.add_rule(Use);
        lex.add_rule(Mod);
        lex.add_rule(Let);
        lex.add_rule(Number);
        lex.add_rule(Ident);
        lex.add_rule(Fun);
        lex.add_rule(Struct);
        lex.add_rule(EqOp);
        Self { lexer: lex }
    }

    fn run(&mut self, input: &str) -> Result<(), String> {
        self.lexer.run(input)
    }

    fn get_tokens(&self) -> &Vec<String> {
        self.lexer.get_tokens()
    }

    fn get_rules(&self) -> &Vec<(CargryTokens, usize)> {
        self.lexer.get_rules()
    }
}
