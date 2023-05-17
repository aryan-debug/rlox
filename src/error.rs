pub mod error{
    use crate::{token::Token, token_type::TokenType};

    pub fn error(token: Token, message: &str){
        if token.token_type == TokenType::EOF{
            report(token.line, "at end", message);
        }
        else{
            report(token.line, &format!("at '{}'", token.lexeme), message);
        }
    }

    pub fn runtime_error(operator: &Token, message: &str){
        eprintln!("{}\n[line {}]", message, operator.line);
    }

    fn report(line: usize, location: &str, message: &str){
        eprintln!("[line {line}] Error {location}: {message}");
    }
}