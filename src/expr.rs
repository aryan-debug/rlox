use crate::{token::Token, literal::{Literal}};

#[derive(Debug, Clone)]
pub enum Expr{
    Assign(Token, Result<Box<Expr>, ()>),
    Binary(Result<Box<Expr>, ()>, Token, Result<Box<Expr>, ()>),
    Unary(Token, Result<Box<Expr>, ()>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Variable(Token),
    Grouping(Result<Box<Expr>, ()>)
}