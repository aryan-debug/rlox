#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Float(f32),
    Bool(bool),
    Null
}

impl Literal{
    pub fn stringify(literal: Literal) -> String{
        match literal{
            Literal::Null => String::from("nil"),
            Literal::String(value) => value,
            Literal::Float(value) => value.to_string(),
            Literal::Bool(value) => value.to_string(),
        }
    }
}