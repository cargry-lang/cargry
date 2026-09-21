/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

enum PType<'a, Tokens, F>
where
    F: Fn() -> PdkParser<'a, Tokens, F>,
{
    Eat(Tokens),
    Opt(&'a PdkParser<'a, Tokens, F>),
    Call(&'a PdkParser<'a, Tokens, F>),
    CallFun(F),
}

pub fn pinit<'a, Tokens, F>() -> PdkParser<'a, Tokens, F>
where
    F: Fn() -> PdkParser<'a, Tokens, F>,
{
    PdkParser::new()
}

pub struct PdkParser<'a, Tokens, F>
where
    F: Fn() -> PdkParser<'a, Tokens, F>,
{
    queue: Vec<PType<'a, Tokens, F>>,
}
impl<'a, Tokens, F> PdkParser<'a, Tokens, F>
where
    F: Fn() -> PdkParser<'a, Tokens, F>,
{
    pub fn new() -> Self {
        Self { queue: vec![] }
    }

    pub fn token(mut self, token: Tokens) -> Self {
        self.queue.push(PType::Eat(token));
        self
    }

    pub fn opt(mut self, pdk_parser: &'a PdkParser<'a, Tokens, F>) -> Self {
        self.queue.push(PType::Opt(pdk_parser));
        self
    }

    pub fn call(mut self, pdk_parser: &'a PdkParser<'a, Tokens, F>) -> Self {
        self.queue.push(PType::Call(pdk_parser));
        self
    }

    pub fn call_f(mut self, f: F) -> Self {
        self.queue.push(PType::CallFun(f));
        self
    }
}
