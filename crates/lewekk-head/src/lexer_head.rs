/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Debug};

pub trait LexerManager<T> {
    fn new() -> Self;
    fn run(&mut self, input: &str) -> Result<(), String>;
    fn get_tokens(&self) -> &Vec<String>;
    fn get_rules(&self) -> &Vec<(T, usize)>;
}

pub trait LexRule<T>: LexClone<T> + LexMapping<T> + LexDisplay {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String>;
}

pub trait LexMapping<T> {
    fn ltoken(&self) -> T;
}

pub trait LexClone<T> {
    fn clone_box(&self) -> Box<dyn LexRule<T>>;
}

pub trait LexDisplay {
    fn get_name(&self) -> &str;
}

impl<T> Clone for Box<dyn LexRule<T>> {
    fn clone(&self) -> Box<dyn LexRule<T>> {
        self.clone_box()
    }
}

impl<T> Debug for Box<dyn LexRule<T>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        write!(f, "{}", self.get_name())?;
        write!(f, " ]")?;
        Ok(())
    }
}
