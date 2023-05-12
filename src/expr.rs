use crate::{token::Token, literal::{Literal, self}};

#[derive(Clone)]
pub enum Expr{
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Literal),
    Grouping(Box<Expr>)
}

impl Expr{
    pub fn print(expression: &Expr) -> String {
        match expression{
            Expr::Binary(left, operator, right) => parenthesize(operator.lexeme.clone(), vec![&*left, &*right]),
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
        result += &Expr::print(expr);
    }
    result += ")";
    println!("{result}");
    result
}
