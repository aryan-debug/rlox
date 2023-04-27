use crate::{token_type::{TokenType}, literal::Literal};

#[derive(Debug)]
pub struct Token{
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize
}

impl Token{
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token{
        Self{token_type, lexeme, literal, line}
    }
}