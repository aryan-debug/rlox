use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{literal::Literal, token::Token, error_handler::error};

#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: None
        }))
    }

    pub fn from_existing(environemnt: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let enclosing = Some(environemnt);
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing,
        }))
    }


    pub fn define(&mut self, name: String, value: Option<Literal>) {
        let value = if let Some(val) = value {
            val
        }
        else {
            Literal::Null
        };
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Option<Literal> {
        if self.values.contains_key(&name.lexeme) {
            return Some(self.values.get(&name.lexeme).unwrap().clone());
        }
        
        match &self.enclosing {
            Some(environment) => return environment.borrow().get(name),
            None => return {
                error::runtime_error(name, format!("Undefined variable {}.", name.lexeme).as_str());
                None
            }
        }

    }

    pub fn assign(&mut self, name: &Token, value: &Literal) {
        if let Some(name) = self.values.get_mut(&name.lexeme) {
            *name = value.clone();
            return;
        }
        match &mut self.enclosing {
            Some(environment) => environment.borrow_mut().assign(name, value),
            None => error::runtime_error(name, &format!("Undefined variable '{}'.",name.lexeme)),
        }
    }
}