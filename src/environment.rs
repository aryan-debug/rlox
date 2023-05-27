use std::collections::HashMap;

use crate::{literal::Literal, token::Token, error::error};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(environemnt: Option<&Environment>) -> Environment {
        match environemnt {
            Some(env) => return Environment {values: HashMap::new(), enclosing: Some(Box::new(env.clone()))},
            None => return Environment { values: HashMap::new(), enclosing: None}
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&Literal, ()> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap());
        }
        
        match &self.enclosing {
            Some(environment) => return environment.get(name),
            None => return Err(error::runtime_error(&name, format!("Undefined variable {}.", name.lexeme).as_str()))
        }

    }

    pub fn assign(&mut self, name: &Token, value: &Literal) {
        if let Some(name) = self.values.get_mut(&name.lexeme) {
            *name = value.clone();
            return;
        }

        match &mut self.enclosing {
            Some(environment) => environment.assign(name, value),
            None => error::runtime_error(name, &format!("Undefined variable '{}'.",name.lexeme)),
        }
    }
}