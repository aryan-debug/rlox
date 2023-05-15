use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret<'a>(&'a self, expression: &'a Expr) -> Literal {
        let value = self.evaluate(&expression);
        return value;
    }

    fn accept<'a>(&'a self, expression: &'a Expr) -> Literal{
        match expression{
            Expr::Binary(left, operator, right) => self.handle_binary(left.as_ref().unwrap(), operator, right.as_ref().unwrap()),
            Expr::Unary(operator, right) => self.handle_unary(operator, right.as_ref().unwrap()),
            Expr::Literal(literal) => literal.clone(),
            Expr::Grouping(value) => self.evaluate(value.as_ref().unwrap()),
        }
    }

    fn evaluate<'a>(&'a self, expr: &'a Expr) -> Literal {
        self.accept(expr)
    }

    fn handle_binary<'a>(&self, left: &'a Expr, operator: &Token, right: &'a Expr) -> Literal{
        let left = self.evaluate(left);
        let right = self.evaluate(right);
        match (left, right) {
            (Literal::Float(left), Literal::Float(right)) => {
                match operator.token_type{
                    TokenType::Plus => {
                        Literal::Float(left + right)
                    }
                    TokenType::Minus => {
                        Literal::Float(left - right)
                    },
                    TokenType::Slash => {
                        Literal::Float(left / right)
                    },
                    TokenType::Star => {
                        Literal::Float(left * right)
                    }
        
                    _ => unimplemented!()
                }
            },
            (Literal::String(left), Literal::String(right)) => {
                match operator.token_type{
                    TokenType::Plus => {
                        Literal::String(left + &right)
                    }
                    _ => unimplemented!()
                }
            }
            (_, _) => todo!()
        }
    }

    fn handle_unary<'a>(&self, operator: &Token, expr: &'a Expr) -> Literal{
        let right = self.evaluate(expr);
        match (&operator.token_type, &right){
            (TokenType::Minus, Literal::Float(value)) => {
                return Literal::Float(-value);
            }
            (TokenType::Bang, _) => {
                return Literal::Bool(!self.is_truthy(right))
            }
            (_, _) => todo!()
        }
    }

    fn is_truthy(&self, literal: Literal) -> bool{
        match literal{
            Literal::Null => false,
            Literal::Bool(value) => value,
            _ => true
        }
    }
}
