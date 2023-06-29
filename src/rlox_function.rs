use std::{rc::Rc, cell::RefCell};

use crate::{stmt::Stmt, interpreter::{Interpreter}, literal::{Literal, TCallable}, environment::Environment};
#[derive(Debug)]
pub struct RloxFunction {
    declaration: Stmt,
    closure: Rc<RefCell<Environment>>,
}

impl RloxFunction {
    pub fn new(declaration: Stmt, closure: Rc<RefCell<Environment>>) -> Self {
        RloxFunction { declaration , closure }
    }

    
}

impl TCallable for RloxFunction {
    fn arity(&self) -> usize {
        if let Stmt::Function(_, params, _) = &self.declaration {
            params.len()
        } else { 0 }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Option<Literal>]) -> Option<Literal> {
        let environment = Environment::from_existing(Rc::clone(&self.closure));
        if let Stmt::Function(_, params, body) = &self.declaration {
            for i in 0..params.len() {
                environment.borrow_mut().define(params.get(i)?.lexeme.clone(), arguments.get(i).unwrap().clone());
            }
            if let Err(value) = interpreter.execute_block(body, environment) {
                return value;
            }
        }
        None
    }
}