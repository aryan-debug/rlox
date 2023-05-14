use crate::{token::Token, literal::{Literal}};

#[derive(Debug, Clone)]
pub enum Expr{
    Binary(Result<Box<Expr>, ()>, Token, Result<Box<Expr>, ()>),
    Unary(Token, Result<Box<Expr>, ()>),
    Literal(Literal),
    Grouping(Result<Box<Expr>, ()>)
}

impl Expr{
    pub fn print(expression: &Expr) -> String {
        match expression{
            Expr::Binary(left, operator, right) => parenthesize(operator.lexeme.clone(), vec![&left.as_ref().unwrap(), &right.as_ref().unwrap()]),
            Expr::Unary(operator, right) => todo!(),
            Expr::Literal(literal) => match_literal(literal),
            Expr::Grouping(value) => todo!()

        }
    }
}

fn match_literal(literal: &Literal) -> String{
    match literal{
        Literal::Float(value) => value.to_string(),
        _ => "hello".to_string()
    }
}

fn parenthesize(name: String, exprs: Vec<&Expr>) -> String{
    let mut result = String::new();
    result += &format!("( {name}");
    for expr in exprs{
        result += " ";
        result += &Expr::print(&expr);
    }
    result += ")";
    println!("{result}");
    result
}
