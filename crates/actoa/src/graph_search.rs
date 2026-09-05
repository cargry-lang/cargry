/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::graph::Actoa;
use crate::utils::{Relation, TypeExpr};

pub struct ActoaSearch<TE, Rel, F1>
where
    TE: TypeExpr,
    Rel: Relation,
    F1: Fn() -> Rel,
{
    actoa: Actoa<TE, Rel, F1>,
    type_list: Vec<String>,
}
impl<TE, Rel, F1> ActoaSearch<TE, Rel, F1>
where
    TE: TypeExpr,
    Rel: Relation,
    F1: Fn() -> Rel,
{
    pub fn new(actoa: Actoa<TE, Rel, F1>) -> Self {
        let size = actoa.len();
        Self {
            actoa,
            type_list: vec![String::new(); size],
        }
    }

    pub fn search(&mut self) {}
}
