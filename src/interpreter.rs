use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType, error::error, stmt::Stmt, environment::{Environment}};
use std::mem;
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
            },
            Stmt::Block(statements) => self.execute_block(statements, &mut Environment::from_existing(self.environment.clone())),
            Stmt::If(condition, then_branch, else_branch) => {
                let condition_result = self.evaluate(condition).unwrap();
                if self.is_truthy(&condition_result) {
                    self.execute(then_branch)
                }
                else if else_branch.is_some()  {
                    self.execute(else_branch.as_ref().unwrap())
                }
            }
        }
    }

    fn accept_expression<'a>(&'a mut self, expression: &'a Expr) -> Result<Literal, ()>{
        match expression{
            Expr::Binary(left, operator, right) => self.handle_binary(left.as_ref().unwrap(), operator, right.as_ref().unwrap()),
            Expr::Unary(operator, right) => self.handle_unary(operator, right.as_ref().unwrap()),
            Expr::Literal(literal) => Ok(literal.clone()),
            Expr::Grouping(value) => self.evaluate(value.as_ref().unwrap()),
            Expr::Variable(value) => self.environment.get(value).cloned(),
            Expr::Assign(name, value) => {
                let value = self.evaluate((*value).as_ref().unwrap());
                self.environment.assign(name, value.as_ref().unwrap());
                return value;
            },
            Expr::Logical(left, operator, right) => {
                let left = self.evaluate(left);
                if let TokenType::Or = operator.token_type {
                    if self.is_truthy(left.as_ref().unwrap()) { return left; };
                }
                else {
                    if !self.is_truthy(left.as_ref().unwrap()) { return left };
                }

                return self.evaluate(right);
            },
        }
    }

    fn evaluate<'a>(&'a mut self, expr: &'a Expr) -> Result<Literal, ()> {
        self.accept_expression(expr)
    }

    fn execute(&mut self, stmt: &Stmt) {
        self.accept_statement(stmt)
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>, mut environment: &mut Environment) {
        mem::swap(&mut self.environment, &mut environment);

        for statement in stmts {
            self.execute(statement)
        }

        self.environment = environment.clone();
    }

    fn handle_binary<'a>(&mut self, left: &'a Expr, operator: &Token, right: &'a Expr) -> Result<Literal, ()>{
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

    fn handle_unary<'a>(&mut self, operator: &Token, expr: &'a Expr) -> Result<Literal, ()>{
        let right = self.evaluate(expr).unwrap();
        match (&operator.token_type, &right){
            (TokenType::Minus, Literal::Float(value)) => {
                return Ok(Literal::Float(-value));
            }
            (TokenType::Bang, _) => {
                return Ok(Literal::Bool(!self.is_truthy(&right)))
            }
            _ => {Err(error::runtime_error(operator, "Operand must be a number"))}
        }
    }

    fn is_truthy(&self, literal: &Literal) -> bool{
        match literal{
            Literal::Null => false,
            Literal::Bool(value) => *value,
            _ => true
        }
    }

    fn is_equal<T: PartialEq>(&self, left: T, right: T) -> bool{
        left == right
    }
}
