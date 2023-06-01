use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType, error_handler::error, stmt::Stmt, };

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
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            else {
                self.synchronize();
            }
        }
        statements

    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_tokens(vec![TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect variable name").unwrap();
        let mut initializer = None;
        if self.match_tokens(vec![TokenType::Equal]) {
            if let Some(expression) = self.expression() {
                initializer = Some(*expression);
            }
            else {
                return None;
            }
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.").unwrap();
        Some(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.match_tokens(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_tokens(vec![TokenType::LeftBrace]) {
            return Some(Stmt::Block(self.block()));
        }
        if self.match_tokens(vec![TokenType::If]) {
            return Some(self.if_statement());
        }
        if self.match_tokens(vec![TokenType::While]) {
            return Some(self.while_statement());
        }
        if self.match_tokens(vec![TokenType::For]) {
            return Some(self.for_statement());
        }
        self.expression_statement()
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after 'if'.");

        let then_branch = self.statement();

        let mut else_branch = None;

        if self.match_tokens(vec![TokenType::Else]) {

            else_branch = self.statement().map(Box::new);

        }

        

        Stmt::If(*condition.unwrap(), Box::new(then_branch.unwrap()), else_branch)
    }

    fn while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' before 'while'");
        let condition = *self.expression().unwrap();
        self.consume(TokenType::RightParen, "Expect ')' after 'conditon'");

        let body = self.statement().unwrap();

        Stmt::While(condition, Box::new(body))
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
        let initializer;

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

        if let Some(condition) = condition {
            body = Stmt::While(*condition, Box::new(body));
        }
        else {
            body = Stmt::While(Expr::Literal(Literal::Bool(true)), Box::new(body));
        }

        if initializer.is_some() {
            body = Stmt::Block(vec![initializer.unwrap(), body]);
        }

        body
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let value = self.expression();
        if let Some(value) = value {
            self.consume(TokenType::Semicolon, "Expect ';' after value.");
            return Some(Stmt::Print(*value));
        }
        None
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression();
        if let Some(expr) = expr {
            self.consume(TokenType::Semicolon, "Expect ';' after expression.").unwrap();
            return Some(Stmt::Expr(*expr));
        }
        None
    }

    fn expression(&mut self) -> Option<Box<Expr>> {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Box<Expr>> {
        let expr = self.or();

        if self.match_tokens(vec![TokenType::Equal]) {
            let equals = self.previous();
            if let Some(value) = self.assignment() {
                if let Expr::Variable(name) = *expr.clone().unwrap() {
                    return Some(Box::new(Expr::Assign(name, value)));
                }
            }
            else {
                error::error(equals, "Invalid assignment target.");
                return None;
            }
        }

        expr
    }

    fn or(&mut self) -> Option<Box<Expr>> {
        if let Some(mut expr) = self.and() {
            while self.match_tokens(vec![TokenType::Or]) {
                let operator = self.previous();
                if let Some(right) = self.and() {
                    expr = Box::new(Expr::Logical(expr, operator, right));
                }
            }
    
            Some(expr)
        }
        else {
            None
        }
    }

    fn and(&mut self) -> Option<Box<Expr>> {
        if let Some(mut expr) = self.equality() {
            while self.match_tokens(vec![TokenType::And]) {
                let operator = self.previous();
                if let Some(right) = self.equality() {
                    expr = Box::new(Expr::Logical(expr, operator, right));
                }
            }
    
            Some(expr)
        }
        else {
            None
        }
    }

    fn equality(&mut self) -> Option<Box<Expr>> {
        if let Some(mut expr) = self.comparison() {
            while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
                let operator = self.previous();
                if let Some(right) = self.comparison() {
                    expr = Box::new(Expr::Binary(expr, operator, right));
                }
            }
    
            Some(expr)
        }
        else {
            None
        }
        
    }

    fn comparison(&mut self) -> Option<Box<Expr>> {
        if let Some(mut expr) = self.term() {
            while self.match_tokens(vec![
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ]) {
                let operator = self.previous();
                if let Some(right) = self.term() {
                    expr = Box::new(Expr::Binary(expr, operator, right));
                }
            }
            Some(expr)
        }
        else {
            None
        }
    }

    fn term(&mut self) -> Option<Box<Expr>> {
        if let Some(mut expr) = self.factor() {
            while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
                let operator = self.previous();
                if let Some(right) = self.factor() {
                    expr = Box::new(Expr::Binary(expr, operator, right));
                }
            }
    
            Some(expr)
        }
        else {
            None
        }
    }

    fn factor(&mut self) -> Option<Box<Expr>> {
        if let Some(mut expr) = self.unary() {
            while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
                let operator = self.previous();
                if let Some(right) = self.unary() {
                    expr = Box::new(Expr::Binary(expr, operator, right));
                }
            }
    
            Some(expr)
        }
        else {
            None
        }
    }

    fn unary(&mut self) -> Option<Box<Expr>> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            if let Some(right) = self.unary() {
                return Some(Box::new(Expr::Unary(operator, right)));
            }
            else {
                self.error(operator, "Expected expression on the right hand side.");
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Option<Box<Expr>> {
        if self.match_tokens(vec![TokenType::False]) {
            Some(Box::new(Expr::Literal(Literal::Bool(false))))
        }
        else if self.match_tokens(vec![TokenType::True]) {
            Some(Box::new(Expr::Literal(Literal::Bool(true))))
        }
        else if self.match_tokens(vec![TokenType::Nil]) {
            Some(Box::new(Expr::Literal(Literal::Null)))
        }

        else if self.match_tokens(vec![TokenType::Number, TokenType::String]) {
            Some(Box::new(Expr::Literal(self.previous().literal.unwrap())))
        }

        else if self.match_tokens(vec![TokenType::LeftParen]) {
            if let Some(expr) = self.expression() {
                return Some(Box::new(Expr::Grouping(expr)));
            }
            else {
                self.consume(TokenType::RightParen, "Expect ')' after expression");
                None
            }
        }
        else if self.match_tokens(vec![TokenType::Identifier]) {
            return Some(Box::new(Expr::Variable(self.previous())));
        }
        else {
            self.error(self.peek(), "Expect expression.");
            None
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.match_token(token_type) {
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<Token> {
        if self.check(token_type) {
            return Some(self.advance());
        }
        self.error(self.peek(), message);
        None

    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
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

