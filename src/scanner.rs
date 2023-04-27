use crate::{token_type::{TokenType}, token::Token, literal::Literal, keywords::KEYWORDS, code_runner::{CodeRunner}};

pub struct Scanner<'a>{
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    code_runner: &'a mut CodeRunner
}

impl Scanner<'_>{
    pub fn new(source: String, code_runner: &mut CodeRunner) -> Scanner{
        Scanner{source, tokens: vec![], start: 0, current: 0, line: 1, code_runner}
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token>{
        while !self.is_at_end(){
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::new(), Option::None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self){
        let c = self.advance();
        match c {
            '(' => self.add_token_with_no_literal(TokenType::LeftParen),
            ')' => self.add_token_with_no_literal(TokenType::RightParen),
            '{' => self.add_token_with_no_literal(TokenType::LeftBrace),
            '}' => self.add_token_with_no_literal(TokenType::RightBrace),
            ',' => self.add_token_with_no_literal(TokenType::Comma),
            '.' => self.add_token_with_no_literal(TokenType::Dot),
            '-' => self.add_token_with_no_literal(TokenType::Minus),
            '+' => self.add_token_with_no_literal(TokenType::Plus),
            ';' => self.add_token_with_no_literal(TokenType::Semicolon),
            '*' => self.add_token_with_no_literal(TokenType::Star),
            '!' => self.add_matched_token('=', TokenType::BangEqual, TokenType::Bang, Option::None),
            '=' => self.add_matched_token('=', TokenType::EqualEqual, TokenType::Equal, Option::None),
            '<' => self.add_matched_token('=', TokenType::LessEqual, TokenType::Less, Option::None),
            '>' => self.add_matched_token('=', TokenType::GreaterEqual, TokenType::Greater, Option::None),
            '/' => {
                if self.match_token('/'){
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    } 
                }
                else if self.match_token('*'){
                    self.multiline_comment();
                }
                else{
                    self.add_token_with_no_literal(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ => {
                if self.is_alphanumeric(c){
                    self.identifier();
                }
                else{
                    self.code_runner.error(self.line, "Unexpected character");
                }
            }

        }
    }

    fn advance(&mut self) -> char{
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>){
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    fn add_token_with_no_literal(&mut self, token_type: TokenType){
       
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text.to_string(), Option::None, self.line));
    }

    fn match_token(&mut self, expected: char) -> bool{
        if self.is_at_end(){
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected{
            return false
        }
        self.current += 1;
        true
    }

    fn add_matched_token(&mut self, expected: char, matched_token: TokenType, unmatched_token: TokenType, literal: Option<Literal>){
        let token_type = if self.match_token(expected) { matched_token } else{ unmatched_token };
        self.add_token(token_type, literal);

    }

    fn peek(&self) -> char{
        if self.is_at_end(){
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char{
        if self.current + 1 >= self.source.len(){
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self){
        while self.peek() != '"' && !self.is_at_end(){
            if self.peek() == '\n'{
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end(){
            self.code_runner.error(self.line, "Unterminated string");
        }
        else{
            self.advance();

            let value = &self.source[self.start + 1..self.current - 1];
            self.add_token(TokenType::String, Option::Some(Literal::String(value.to_string())));
        }
    }

    fn number(&mut self){
        while self.is_digit(self.peek()){
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()){
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let number = self.source[self.start..self.current].parse::<f32>().unwrap();
        self.add_token(TokenType::Number, Some(Literal::Float(number)))
    }

    fn identifier(&mut self){
        while self.is_alphanumeric(self.peek()){
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(text).unwrap_or(&TokenType::Identifier);
        self.add_token_with_no_literal(token_type.clone());
    }

    fn multiline_comment(&mut self){
        while self.peek() != '*' && !self.is_at_end(){
            if self.peek() == '\n'{
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end(){
            self.code_runner.error(self.line, "Unterminated multiline comment");
        }
        else{
            self.advance();

            if self.peek() == '/'{
                self.advance();
            }
            
            else{
                self.code_runner.error(self.line, "Unterminated multiline comment");
            }
        }
    }

    fn is_digit(&self, c: char) -> bool{
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool{
      (c.to_ascii_lowercase() >= 'a' && c.to_ascii_lowercase() <= 'z') || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool{
        self.is_alpha(c) || self.is_digit(c)
    }
    
    fn is_at_end(&self) -> bool{
        self.current >=  self.source.len()
    }

}