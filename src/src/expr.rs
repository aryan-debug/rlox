use crate::{token::Token, literal::{Literal}};

#[derive(Debug, Clone)]
pub enum Expr{
    Binary(Result<Box<Expr>, ()>, Token, Result<Box<Expr>, ()>),
    Unary(Token, Result<Box<Expr>, ()>),
    Literal(Literal),
    Variable(Token),
    Grouping(Result<Box<Expr>, ()>)
}