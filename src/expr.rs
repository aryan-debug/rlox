use crate::{token::Token, literal::{Literal}};

#[derive(Debug, Clone)]
pub enum Expr{
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Variable(Token),
    Grouping(Box<Expr>)
}