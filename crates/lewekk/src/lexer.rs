/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lewekk_head::LexRule;

pub struct Lexer<T> {
    rules: Vec<Box<dyn LexRule<T>>>,
    ign_rule: Option<Box<dyn LexRule<T>>>,
    tokens: Vec<String>,
    rule_pos: Vec<(T, usize)>,
}

impl<T> Lexer<T> {
    pub fn new(rule: Option<impl LexRule<T> + Clone + 'static>) -> Self {
        Self {
            rules: vec![],
            ign_rule: if let Some(rule) = rule {
                Some(Box::new(rule))
            } else {
                None
            },
            tokens: vec![],
            rule_pos: vec![],
        }
    }

    pub fn add_rule(&mut self, rule: impl LexRule<T> + Clone + 'static) {
        self.rules.push(Box::new(rule));
    }

    fn excute(&mut self, input: &String) -> Result<(String, Vec<String>), String> {
        let index = self.tokens.len();
        for rule in self.rules.iter_mut() {
            let lparsed = rule.lparse(input);
            if let Ok((rest, vec)) = lparsed {
                self.rule_pos.push((rule.ltoken(), index));
                return Ok((rest, vec));
            }
        }
        Err(String::from(""))
    }

    pub fn run(&mut self, input: &str) -> Result<(), String> {
        let mut input = input.to_string();
        let exist_ign_rule = self.ign_rule.is_some();
        if exist_ign_rule {
            let (rest, _) = self.ign_rule.as_ref().unwrap().lparse(&input)?;
            input = rest;
        }
        while !input.is_empty() {
            let (rest, vec) = self.excute(&input)?;
            for item in vec.into_iter() {
                self.tokens.push(item);
            }
            input = rest;
            if exist_ign_rule {
                let (rest, _) = self.ign_rule.as_ref().unwrap().lparse(&input)?;
                input = rest;
            }
        }
        Ok(())
    }

    pub fn get_tokens(&self) -> &Vec<String> {
        &self.tokens
    }

    pub fn get_rules(&self) -> &Vec<(T, usize)> {
        &self.rule_pos
    }
}
