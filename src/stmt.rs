use crate::{expr::Expr, token::Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Function(Token, Vec<Token>, Vec<Stmt>),
    Return(Token, Option<Expr>)
}