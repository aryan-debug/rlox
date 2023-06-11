use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType, error_handler::error, stmt::Stmt, };

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0, had_error: false }
    }
    
    pub fn get_had_err(&self) -> bool {
        self.had_error
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            let token = self.peek();

            match token.token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            else {
                self.had_error = true;
                self.synchronize();
            }
        }
        statements

    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_token(TokenType::Var) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect variable name")?;
        let mut initializer = None;
        if self.match_token(TokenType::Equal) {
            let expression = self.expression()?;
            initializer = Some(*expression);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Some(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.match_token(TokenType::Print) {
            return self.print_statement();
        }
        if self.match_token(TokenType::LeftBrace) {
            return self.block();
        }
        if self.match_token(TokenType::If) {
            return self.if_statement();
        }
        if self.match_token(TokenType::While) {
            return self.while_statement();
        }
        if self.match_token(TokenType::For) {
            return self.for_statement();
        }
        self.expression_statement()
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        if let Some(condition) = self.expression() {
            self.consume(TokenType::RightParen, "Expect ')' after 'if'.");

            if let Some(then_branch) = self.statement() {
                let mut else_branch = None;

                if self.match_token(TokenType::Else) {

                    else_branch = self.statement().map(Box::new);

                }

                Some(Stmt::If(*condition, Box::new(then_branch), else_branch))
            }
            else {

                None
            }
        }
        else {
            None
        }
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' before 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'conditon'")?;
        self.statement().map(|body| Stmt::While(*condition, Box::new(body)))
    }

    fn block(&mut self) -> Option<Stmt> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(declaration) = self.declaration() {
                statements.push(declaration);
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Some(Stmt::Block(statements))
    }

    fn for_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");
        let initializer;

        if self.match_token(TokenType::Semicolon) {
            initializer = None;
        }
        else if self.match_token(TokenType::Var) {
            initializer = self.var_declaration();
        }
        else if let Some(init) = self.expression_statement() {
            initializer = Some(init);
        }
        else {
            initializer = None;
        }

        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            if let Some(cond) = self.expression() {
                condition = Some(cond);
            }
        }

        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

        let mut increment = None;

        if !self.check(TokenType::RightParen) {
            if let Some(incr) = self.expression() {
                increment = Some(incr);
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after 'for'.");

        let mut body: Option<Stmt>;

        if let Some(stmt) = self.statement() {
            body = Some(stmt);

            if let Some(inc) = increment {
                body = Some(Stmt::Block(vec![body?, Stmt::Expr(*inc)]));
            }

            if let Some(condition) = condition {
                body = Some(Stmt::While(*condition, Box::new(body?)));
            }
            else {
                body = Some(Stmt::While(Expr::Literal(Literal::Bool(true)), Box::new(body?)));
            }

            if initializer.is_some() {
                body = Some(Stmt::Block(vec![initializer?, body?]));
            }
        }
        else {
            body = None
        }

        body
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        if let Some(value) = self.expression() {
            self.consume(TokenType::Semicolon, "Expect ';' after value.");
            return Some(Stmt::Print(*value));
        }
        None
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        if let Some(expr) = self.expression() {
            self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
            return Some(Stmt::Expr(*expr));
        }
        None
    }

    fn expression(&mut self) -> Option<Box<Expr>> {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Box<Expr>> {
        let expr = self.or();

        if self.match_token(TokenType::Equal) {
            let equals = self.previous();
            if let Some(value) = self.assignment() {
                if let Expr::Variable(name) = *expr.clone()? {
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
            while self.match_token(TokenType::Or) {
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
            while self.match_token(TokenType::And) {
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
        if self.match_token(TokenType::False) {
            return Some(Box::new(Expr::Literal(Literal::Bool(false))));
        }
        if self.match_token(TokenType::True) {
            return Some(Box::new(Expr::Literal(Literal::Bool(true))));
        }
        if self.match_token(TokenType::Nil) {
            return Some(Box::new(Expr::Literal(Literal::Null)));
        }

        if self.match_tokens(vec![TokenType::Number, TokenType::String]) {
            return Some(Box::new(Expr::Literal(self.previous().literal?)));
        }

        if self.match_token(TokenType::LeftParen) {
            if let Some(expr) = self.expression() {
                self.consume(TokenType::RightParen, "Expect ')' after expression");
                return Some(Box::new(Expr::Grouping(expr)));
            }
            else {
                return None;
            }
        }
        if self.match_token(TokenType::Identifier) {
            return Some(Box::new(Expr::Variable(self.previous())));
        }

        self.error(self.peek(), "Expect expression.");
        None
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{scanner::Scanner};

    fn parse_input(input: &str) -> Vec<Stmt> {
        let mut scanner = Scanner::new(input.to_owned());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.to_vec());
        parser.parse()
    }

    #[test]
    fn primary_true_false_nil() {
        let input = "false; true; nil;";
        let parsed = parse_input(input);
        assert!(parsed.contains(&Stmt::Expr(Expr::Literal(Literal::Bool(false)))));
        assert!(parsed.contains(&Stmt::Expr(Expr::Literal(Literal::Bool(true)))));
        assert!(parsed.contains(&Stmt::Expr(Expr::Literal(Literal::Null))));
    }

    #[test]
    fn primary_() {
        let input = "true;";
        let parsed = parse_input(input);
        assert!(parsed.contains(&Stmt::Expr(Expr::Literal(Literal::Bool(true)))));
    }
    
}
