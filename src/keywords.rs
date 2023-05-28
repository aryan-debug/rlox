use phf::phf_map;
use crate::token_type::TokenType;

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and"    =>  TokenType::And,
    "class"  =>  TokenType::Class,
    "false"  =>  TokenType::False,
    "for"    =>  TokenType::For,
    "fun"    =>  TokenType::Fun,
    "if"     =>  TokenType::If,
    "nil"    =>  TokenType::Nil,
    "or"     =>  TokenType::Or,
    "print"  =>  TokenType::Print,
    "return" =>  TokenType::Return,
    "super"  =>  TokenType::Super,
    "this"   =>  TokenType::This,
    "true"   =>  TokenType::True,
    "var"    =>  TokenType::Var,
    "while"  =>  TokenType::While,
    "else"   => TokenType::Else,
};