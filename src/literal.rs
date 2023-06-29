use std::{fmt::Debug, rc::Rc, time::SystemTime};

use crate::interpreter::Interpreter;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Float(f32),
    Bool(bool),
    Null,
    Callable(Rc<dyn TCallable>),
}

pub trait TCallable: Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Option<Literal>]) -> Option<Literal>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Clock {}

impl TCallable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Option<Literal>]) -> Option<Literal> {
       let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f32 / 1000.0;
       Some(Literal::Float(time))
    }

}

impl PartialEq for Literal { 
    fn eq(&self, other: &Self) -> bool { self == other }
}

impl Literal {
    pub fn stringify(literal: Literal) -> String{
        match literal{
            Literal::Null => String::from("nil"),
            Literal::String(value) => value,
            Literal::Float(value) => value.to_string(),
            Literal::Bool(value) => value.to_string(),
            Literal::Callable(_) => todo!(),
        }
    }
}