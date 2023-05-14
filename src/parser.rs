use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType, error::error};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => todo!(),
            }
        }
    }
    pub fn parse(&mut self) -> Option<Expr> {
        if let Ok(expr) = self.expression(){
            return Some(*expr);
        }
        else{
            return None;
        }
    }

    fn expression(&mut self) -> Result<Box<Expr>, ()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expr>, ()> {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        expr
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ()> {
        let mut expr = self.term();

        while self.match_tokens(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Ok(Box::new(Expr::Binary(expr, operator, right)));
        }
        expr
    }

    fn term(&mut self) -> Result<Box<Expr>, ()> {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        expr
    }

    fn factor(&mut self) -> Result<Box<Expr>, ()> {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        expr
    }

    fn unary(&mut self) -> Result<Box<Expr>, ()> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Ok(Box::new(Expr::Unary(operator, right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, ()> {
        if self.match_tokens(vec![TokenType::False]) {
            return Ok(Box::new(Expr::Literal(Literal::Bool(false))));
        }
        if self.match_tokens(vec![TokenType::True]) {
            return Ok(Box::new(Expr::Literal(Literal::Bool(true))));
        }
        if self.match_tokens(vec![TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal(Literal::Null)));
        }

        if self.match_tokens(vec![TokenType::Number, TokenType::String]) {
            return Ok(Box::new(Expr::Literal(self.previous().literal.unwrap())));
        }

        if self.match_tokens(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression");
            return Ok(Box::new(Expr::Grouping(expr)));
        } else {
            Err(self.error(self.peek(), "Expect expression."))
        }
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ()>{
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))

    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn error(&self, token: Token, message: &str){
        error::error(token, message);
    }
}

