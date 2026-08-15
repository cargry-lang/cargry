use std::fmt::{self, Debug};

pub enum LexResult {
    Some(String),
    Array(Vec<String>),
    None,
}

pub trait LexRule: LexClone + LexIgnore + LexDisplay {
    fn lparse(&mut self, input: &mut String) -> LexResult;
}

pub trait LexerManager {
    fn new() -> Self;
    fn run(&mut self, input: &str);
    fn get_tokens(&self) -> &Vec<String>;
    fn get_rules(&self) -> &Vec<(Box<dyn LexRule>, usize)>;
}

pub trait LexClone {
    fn clone_box(&self) -> Box<dyn LexRule>;
}

pub trait LexIgnore {
    fn is_ignore(&self) -> bool;
}

pub trait LexDisplay {
    fn get_name(&self) -> &str;
}

impl Clone for Box<dyn LexRule> {
    fn clone(&self) -> Box<dyn LexRule> {
        self.clone_box()
    }
}

impl Debug for Box<dyn LexRule> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        write!(f, "{}", self.get_name())?;
        write!(f, " ]")?;
        Ok(())
    }
}
