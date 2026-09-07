/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub enum PStatus {
    Continue,
    End,
}

pub trait PNode<Tokens> {
    fn parse(&self, input: &Vec<Tokens>) -> Result<(PStatus, Vec<Tokens>), String>;
}
