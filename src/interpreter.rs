use std::{rc::Rc, cell::RefCell};

use crate::{expr::Expr, literal::{Literal, Clock}, token::Token, token_type::TokenType, error_handler::error, stmt::Stmt, environment::{Environment}, rlox_function::RloxFunction};
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>
}

impl Interpreter {

    pub fn new() -> Self {
        let environment = Environment::new();
        let interpreter = Interpreter { globals: environment.clone(), environment };
        interpreter.globals.borrow_mut().define("clock".to_string(), Some(Literal::Callable(Rc::new(Clock{}))));
        interpreter
    }

    pub fn interpret<'a>(&'a mut self, stmts: &'a [Stmt]) {
        for stmt in stmts {
            let _ = self.execute(stmt);
        }
    }

    fn accept_statement<'a>(&'a mut self, stmt: &'a Stmt) -> Result<(), Option<Literal>> {
        match stmt {
            Stmt::Expr(expression) => { self.evaluate(expression); Ok(())},
            Stmt::Print(expression) => {
                if let Some(value) = self.evaluate(expression) {
                    println!("{}", Literal::stringify(value));
                }
                Ok(())
            },
            Stmt::Var(token, expression) => {
                let mut value = None;
                if let Some(expression) = expression {
                    value = self.evaluate(expression);
                }

                self.environment.borrow_mut().define(token.lexeme.clone(), value);
                Ok(())
            },
            Stmt::Block(statements) => {
                self.execute_block(statements, Environment::from_existing(Rc::clone(&self.environment)))?;
                Ok(())
            },
            Stmt::If(condition, then_branch, else_branch) => {
                    if let Some(condition_result) = self.evaluate(condition){
                        if self.is_truthy(&condition_result) {
                            self.execute(then_branch)?
                        }
                        else if else_branch.is_some()  {
                            self.execute(else_branch.as_ref().unwrap())?
                        }
                    }
                    Ok(())
            },
            Stmt::While(condition, body) => {
                if let Some(mut result) = self.evaluate(condition){
                    while self.is_truthy(&result) {
                        self.execute(body)?;
                        result = self.evaluate(condition).unwrap();
                    }
                }
                Ok(())
            },
            Stmt::Function(name, _, _) => {
                let function = RloxFunction::new(stmt.clone());
                self.environment.borrow_mut().define(name.lexeme.clone(), Some(Literal::Callable(Rc::new(function))));
                Ok(())
            },
            Stmt::Return(_ , value) => {
                let value = if let Some(value) = value {
                    self.evaluate(value)
                }
                else {
                    None
                };
                Err(value)
            }
        }
    }

    fn accept_expression<'a>(&'a mut self, expression: &'a Expr) -> Option<Literal>{
        match expression{
            Expr::Binary(left, operator, right) => self.handle_binary(left.as_ref(), operator, right.as_ref()),
            Expr::Unary(operator, right) => self.handle_unary(operator, right.as_ref()),
            Expr::Literal(literal) => Some(literal.clone()),
            Expr::Grouping(value) => self.evaluate(value.as_ref()),
            Expr::Variable(value) => self.environment.borrow().get(value),
            Expr::Assign(name, value) => {
                let value = self.evaluate((*value).as_ref());
                self.environment.borrow_mut().assign(name, value.as_ref()?);
                value
            },
            Expr::Logical(left, operator, right) => {
                let left = self.evaluate(left);
                if let TokenType::Or = operator.token_type {
                    if self.is_truthy(left.as_ref()?) { return left; };
                }
                else if !self.is_truthy(left.as_ref()?) { return left };

                self.evaluate(right)
            },
            Expr::Call(callee, paren, arguments) => {
                let callee = self.evaluate(callee)?;
 
                let mut args = vec![];
                for argument in arguments {
                    args.push(self.evaluate(argument.as_ref().unwrap()));
                }
                if let Literal::Callable(callee) = callee {
                    if arguments.len() != callee.arity() {
                        error::runtime_error(paren, format!("Expected {}  arguments, but got {}." , callee.arity(), arguments.len()).as_str())
                    }
                    callee.call(self, &args)
                }
                else {
                    error::runtime_error(paren, "Can only call functions and classes.");
                    None
                }
            },
        }
    }

    fn evaluate<'a>(&'a mut self, expr: &'a Expr) -> Option<Literal> {
        self.accept_expression(expr)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), Option<Literal>> {
        self.accept_statement(stmt)
    }

    pub fn execute_block(&mut self, stmts: &Vec<Stmt>, environment: Rc<RefCell<Environment>>) -> Result<(), Option<Literal>> {
        let previous = self.environment.clone();
        self.environment = environment;

        for statement in stmts {
            if let Err(value) = self.execute(statement) {
                self.environment = previous;
                return Err(value);
            }
        }

        self.environment = previous;
        Ok(())
    }

    fn handle_binary<'a>(&mut self, left: &'a Expr, operator: &Token, right: &'a Expr) -> Option<Literal>{
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
 
        match (&left, &right) {
            
            (Literal::Float(left), Literal::Float(right)) => {
                match &operator.token_type{
                    TokenType::Plus => {
                        Some(Literal::Float(left + right))
                    }
                    TokenType::Minus => {
                        Some(Literal::Float(left - right))
                    },
                    TokenType::Slash => {
                        Some(Literal::Float(left / right))
                    },
                    TokenType::Star => {
                        Some(Literal::Float(left * right))
                    }
                    TokenType::Greater => {
                        Some(Literal::Bool(left > right))
                    }
                    TokenType::GreaterEqual => {
                        Some(Literal::Bool(left >= right))
                    }
                    TokenType::Less => {
                        Some(Literal::Bool(left < right))
                    }
                    TokenType::LessEqual => {
                        Some(Literal::Bool(left <= right))
                    }
                    TokenType::BangEqual => {
                        Some(Literal::Bool(!self.is_equal(left, right)))
                    }
                    TokenType::EqualEqual => {
                        Some(Literal::Bool(self.is_equal(left, right)))
                    }
                    _ =>  { error::runtime_error(operator, "Operands must be numbers"); None }
                }
            },
            (Literal::String(left), Literal::String(right)) => {
                match &operator.token_type{
                    TokenType::Plus => {
                        Some(Literal::String(left.to_owned() + right))
                    }
                    TokenType::BangEqual => {
                        Some(Literal::Bool(!self.is_equal(left, right)))
                    }
                    TokenType::EqualEqual => {
                        Some(Literal::Bool(self.is_equal(left, right)))
                    }
                    _ => { error::runtime_error(operator, "Operands must be two numbers or two strings"); None } 
                }
            }
            (Literal::Bool(left), Literal::Bool(right)) => {
                match operator.token_type{
                    TokenType::BangEqual => {
                        Some(Literal::Bool(!self.is_equal(left, right)))
                    }
                    TokenType::EqualEqual => {
                        Some(Literal::Bool(self.is_equal(left, right)))
                    }
                    _ => unimplemented!()
                }
            }
            (Literal::Null, Literal::Null) => {
                match operator.token_type{
                    TokenType::BangEqual => {
                        Some(Literal::Bool(!self.is_equal(left, right)))
                    }
                    TokenType::EqualEqual => {
                        Some(Literal::Bool(self.is_equal(left, right)))
                    }
                    _ => unimplemented!()
                }
            }
            (_, _) => { error::runtime_error(operator, "Operands must be two numbers or two strings"); None }
        }
    }

    fn handle_unary(&mut self, operator: &Token, expr: &Expr) -> Option<Literal>{
        let right = self.evaluate(expr)?;
        match (&operator.token_type, &right){
            (TokenType::Minus, Literal::Float(value)) => {
                Some(Literal::Float(-value))
            }
            (TokenType::Bang, _) => {
                Some(Literal::Bool(!self.is_truthy(&right)))
            }
            _ => { error::runtime_error(operator, "Operand must be a number"); None }
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
