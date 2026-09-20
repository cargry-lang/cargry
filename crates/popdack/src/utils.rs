/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

enum PType<'a, Tokens> {
    Eat(Tokens),
    Call(&'a PdkParser<'a, Tokens>),
}

pub struct PdkParser<'a, Tokens> {
    queue: Vec<PType<'a, Tokens>>,
}
impl<'a, Tokens> PdkParser<'a, Tokens> {
    pub fn new() -> Self {
        Self { queue: vec![] }
    }

    pub fn token(mut self, token: Tokens) -> Self {
        self.queue.push(PType::Eat(token));
        self
    }

    pub fn call(mut self, pdk_parser: &'a PdkParser<'a, Tokens>) -> Self {
        self.queue.push(PType::Call(pdk_parser));
        self
    }
}
