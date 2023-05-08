use crate::{token_type::{TokenType}, literal::Literal};

#[derive(Debug)]
pub struct Token{
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize
}

impl Token{
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token{
        Self{token_type, lexeme, literal, line}
    }
}