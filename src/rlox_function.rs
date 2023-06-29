use crate::{stmt::Stmt, interpreter::{Interpreter}, literal::{Literal, TCallable}};
#[derive(Debug)]
pub struct RloxFunction {
    declaration: Stmt
}

impl RloxFunction {
    pub fn new(declaration: Stmt) -> Self {
        RloxFunction { declaration }
    }

    
}

impl TCallable for RloxFunction {
    fn arity(&self) -> usize {
        if let Stmt::Function(_, params, _) = &self.declaration {
            params.len()
        } else { 0 }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Option<Literal>]) -> Option<Literal> {
        let environment = &interpreter.globals;
        if let Stmt::Function(name, params, body) = &self.declaration {
            for i in 0..params.len() {
                environment.borrow_mut().define(params.get(i)?.lexeme.clone(), arguments.get(i).unwrap().clone());
            }
            if let Err(value) = interpreter.execute_block(body, environment.clone()) {
                return value;
            }
        }
        None
    }
}