use std::collections::HashMap;

use crate::{literal::Literal, token::Token, error::error};

pub struct Environment {
    values: HashMap<String, Literal>
}

impl Environment {
    pub fn new() -> Environment{
        Environment {values: HashMap::new()}
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&Literal, ()>{
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(&value),
            None => Err(error::runtime_error(&name, format!("Undefined variable {}.", name.lexeme).as_str()))
        }
    }
}