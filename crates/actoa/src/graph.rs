/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashSet;

use crate::utils::{Relation, TypeExpr};

pub struct Actoa<TE, Rel, F>
where
    TE: TypeExpr,
    Rel: Relation,
    F: Fn() -> (usize, Rel),
{
    dag: Vec<HashSet<Rel>>,
    list: Vec<TE>,
    functions: Vec<F>,
}
impl<TE, Rel, F> Actoa<TE, Rel, F>
where
    TE: TypeExpr,
    Rel: Relation,
    F: Fn() -> (usize, Rel),
{
    pub fn new() -> Self {
        Self {
            dag: vec![],
            list: vec![],
            functions: vec![],
        }
    }

    pub fn add_function(&mut self, f: F) {
        self.functions.push(f);
    }

    pub fn add_type_var(&mut self, type_expr: TE) {
        self.dag.push(HashSet::new());
        self.list.push(type_expr);
    }

    pub fn run(&mut self) -> Result<(), String> {
        for f in self.functions.iter() {
            let (target, rel) = f();
            if self.dag.len() <= target {
                return Err(String::from("index was out of range."));
            }
            let set = &mut self.dag[target];
            set.insert(rel);
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}
