use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType, error::error, stmt::Stmt, environment::Environment};

pub struct Interpreter {
    environment: Environment
}

impl Interpreter {

    pub fn new() -> Self {
        return Interpreter { environment: Environment::new() };
    }

    pub fn interpret<'a>(&'a mut self, stmts: &'a [Stmt]) {
        for stmt in stmts {
            self.execute(stmt);
        }
    }

    fn accept_statement<'a>(&'a mut self, stmt: &'a Stmt){
        match stmt {
            Stmt::Expr(expression) => { self.evaluate(expression);},
            Stmt::Print(expression) => {
                let value = self.evaluate(expression);
                println!("{}", Literal::stringify(value.unwrap()));
            },
            Stmt::Var(token, expression) => {
                let mut value = None;
                if let Some(expression) = expression {
                    value = Some(self.evaluate(expression).unwrap());
                }

                self.environment.define(token.lexeme.clone(), value.unwrap());
            }
        }
    }

    fn accept_expression<'a>(&'a self, expression: &'a Expr) -> Result<Literal, ()>{
        match expression{
            Expr::Binary(left, operator, right) => self.handle_binary(left.as_ref().unwrap(), operator, right.as_ref().unwrap()),
            Expr::Unary(operator, right) => self.handle_unary(operator, right.as_ref().unwrap()),
            Expr::Literal(literal) => Ok(literal.clone()),
            Expr::Grouping(value) => self.evaluate(value.as_ref().unwrap()),
            Expr::Variable(value) => self.environment.get(value).cloned()
        }
    }

    fn evaluate<'a>(&'a self, expr: &'a Expr) -> Result<Literal, ()> {
        self.accept_expression(expr)
    }

    fn execute(&mut self, stmt: &Stmt) {
        self.accept_statement(stmt)
    }

    fn handle_binary<'a>(&self, left: &'a Expr, operator: &Token, right: &'a Expr) -> Result<Literal, ()>{
        let left = self.evaluate(left).unwrap();
        let right = self.evaluate(right).unwrap();
 
        match (&left, &right) {
            
            (Literal::Float(left), Literal::Float(right)) => {
                match &operator.token_type{
                    TokenType::Plus => {
                        Ok(Literal::Float(left + right))
                    }
                    TokenType::Minus => {
                        Ok(Literal::Float(left - right))
                    },
                    TokenType::Slash => {
                        Ok(Literal::Float(left / right))
                    },
                    TokenType::Star => {
                        Ok(Literal::Float(left * right))
                    }
                    TokenType::Greater => {
                        Ok(Literal::Bool(left > right))
                    }
                    TokenType::GreaterEqual => {
                        Ok(Literal::Bool(left >= right))
                    }
                    TokenType::Less => {
                        Ok(Literal::Bool(left < right))
                    }
                    TokenType::LessEqual => {
                        Ok(Literal::Bool(left <= right))
                    }
                    TokenType::BangEqual => {
                        Ok(Literal::Bool(!self.is_equal(left, right)))
                    }
                    TokenType::EqualEqual => {
                        Ok(Literal::Bool(self.is_equal(left, right)))
                    }
                    _ =>  Err(error::runtime_error(&operator, "Operands must be numbers"))
                }
            },
            (Literal::String(left), Literal::String(right)) => {
                match &operator.token_type{
                    TokenType::Plus => {
                        Ok(Literal::String(left.to_owned() + right))
                    }
                    TokenType::BangEqual => {
                        Ok(Literal::Bool(!self.is_equal(left, right)))
                    }
                    TokenType::EqualEqual => {
                        Ok(Literal::Bool(self.is_equal(left, right)))
                    }
                    _ => Err(error::runtime_error(&operator, "Operands must be two numbers or two strings"))
                }
            }
            (Literal::Bool(left), Literal::Bool(right)) => {
                match operator.token_type{
                    TokenType::BangEqual => {
                        Ok(Literal::Bool(!self.is_equal(left, right)))
                    }
                    TokenType::EqualEqual => {
                        Ok(Literal::Bool(self.is_equal(left, right)))
                    }
                    _ => unimplemented!()
                }
            }
            (Literal::Null, Literal::Null) => {
                match operator.token_type{
                    TokenType::BangEqual => {
                        Ok(Literal::Bool(!self.is_equal(left, right)))
                    }
                    TokenType::EqualEqual => {
                        Ok(Literal::Bool(self.is_equal(left, right)))
                    }
                    _ => unimplemented!()
                }
            }
            (_, _) => Err(error::runtime_error(&operator, "Operands must be two numbers or two strings"))
        }
    }

    fn handle_unary<'a>(&self, operator: &Token, expr: &'a Expr) -> Result<Literal, ()>{
        let right = self.evaluate(expr).unwrap();
        match (&operator.token_type, &right){
            (TokenType::Minus, Literal::Float(value)) => {
                return Ok(Literal::Float(-value));
            }
            (TokenType::Bang, _) => {
                return Ok(Literal::Bool(!self.is_truthy(right)))
            }
            _ => {Err(error::runtime_error(operator, "Operand must be a number"))}
        }
    }

    fn is_truthy(&self, literal: Literal) -> bool{
        match literal{
            Literal::Null => false,
            Literal::Bool(value) => value,
            _ => true
        }
    }

    fn is_equal<T: PartialEq>(&self, left: T, right: T) -> bool{
        left == right
    }
}
