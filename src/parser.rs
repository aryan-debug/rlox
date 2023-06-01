use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType, error::error, stmt::Stmt, };

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
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            if let Ok(stmt) = self.declaration() {
                statements.push(stmt);
            }
            else {
                self.synchronize();
            }
        }
        statements

    }

    fn declaration(&mut self) -> Result<Stmt, ()> {
        if self.match_tokens(vec![TokenType::Var]) {
            return self.var_declaration();
        }
        return self.statement();
    }

    fn var_declaration(&mut self) -> Result<Stmt, ()> {
        let name = self.consume(TokenType::Identifier, "Expect variable name").unwrap();
        let mut initializer = None;
        if self.match_tokens(vec![TokenType::Equal]) {
            if let Ok(expression) = self.expression() {
                initializer = Some(*expression);
            }
            else {
                return Err(());
            }
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.").unwrap();
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, ()> {
        if self.match_tokens(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_tokens(vec![TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()));
        }
        if self.match_tokens(vec![TokenType::If]) {
            return Ok(self.if_statement());
        }
        if self.match_tokens(vec![TokenType::While]) {
            return Ok(self.while_statement());
        }
        if self.match_tokens(vec![TokenType::For]) {
            return Ok(self.for_statement());
        }
        return self.expression_statement();
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after 'if'.");

        let then_branch = self.statement();
        
        let mut else_branch:Option<Box<Stmt>> = None;

        if self.match_tokens(vec![TokenType::Else]) {

            else_branch = if let Ok(_else) = self.statement() {
                Some(Box::new(_else))
            } else { None }

        }
        else {
            else_branch = None;
        }

        

        Stmt::If(*condition.unwrap(), Box::new(then_branch.unwrap()), else_branch)
    }

    fn while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' before 'while'");
        let condition = *self.expression().unwrap();
        self.consume(TokenType::RightParen, "Expect ')' after 'conditon'");

        let body = self.statement().unwrap();

        return Stmt::While(condition, Box::new(body));
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        statements
    }

    fn for_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");
        let mut initializer = None;

        if self.match_tokens(vec![TokenType::Semicolon]) {
            initializer = None;
        }
        else if self.match_tokens(vec![TokenType::Var]) {
            initializer = Some(self.var_declaration().unwrap());
        }
        else {
            initializer = Some(self.expression_statement().unwrap());
        }

        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression().unwrap());
        }

        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

        let mut increment = None;

        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression().unwrap());
        }

        self.consume(TokenType::RightParen, "Expected ')' after 'for'.");

        let mut body = self.statement().unwrap();

        if let Some(inc) = increment {
            body = Stmt::Block(vec![body, Stmt::Expr(*inc)])
        }

        if condition.is_none() {
            body = Stmt::While(Expr::Literal(Literal::Bool(true)), Box::new(body));
        }
        else {
            body = Stmt::While(*condition.unwrap(), Box::new(body));
        }

        if initializer.is_some() {
            body = Stmt::Block(vec![initializer.unwrap(), body]);
        }

        return body;




    }

    fn print_statement(&mut self) -> Result<Stmt, ()> {
        let value = self.expression();
        if let Ok(value) = value {
            self.consume(TokenType::Semicolon, "Expect ';' after value.").unwrap();
            return Ok(Stmt::Print(*value));
        }
        Err(())
    }

    fn expression_statement(&mut self) -> Result<Stmt, ()> {
        let expr = self.expression();
        if let Ok(expr) = expr {
            self.consume(TokenType::Semicolon, "Expect ';' after expression.").unwrap();
            return Ok(Stmt::Expr(*expr));
        }
        Err(())
    }

    fn expression(&mut self) -> Result<Box<Expr>, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, ()> {
        let expr = self.or();

        if self.match_tokens(vec![TokenType::Equal]) {
            let equals = self.previous();
            if let Ok(value) = self.assignment() {
                if let Expr::Variable(name) = *expr.clone().unwrap() {
                    return Ok(Box::new(Expr::Assign(name, Ok(value))));
                }
            }
            else {
                error::error(equals, "Invalid assignment target.");
                return Err(());
            }
        }

        return expr;
    }

    fn or(&mut self) -> Result<Box<Expr>, ()> {
        let mut expr = self.and().unwrap();
        while self.match_tokens(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and().unwrap();
            expr = Box::new(Expr::Logical(expr, operator, right));
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Box<Expr>, ()> {
        let mut expr = self.equality().unwrap();

        while self.match_tokens(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality().unwrap();
            expr = Box::new(Expr::Logical(expr, operator, right));
        }

        return Ok(expr);
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
        else if self.match_tokens(vec![TokenType::True]) {
            return Ok(Box::new(Expr::Literal(Literal::Bool(true))));
        }
        else if self.match_tokens(vec![TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal(Literal::Null)));
        }

        else if self.match_tokens(vec![TokenType::Number, TokenType::String]) {
            return Ok(Box::new(Expr::Literal(self.previous().literal.unwrap())));
        }

        else if self.match_tokens(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression").unwrap();
            return Ok(Box::new(Expr::Grouping(expr)));
        }
        else if self.match_tokens(vec![TokenType::Identifier]) {
            return Ok(Box::new(Expr::Variable(self.previous())));
        }
        else {
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

