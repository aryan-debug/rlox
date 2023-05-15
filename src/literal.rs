#[derive(Debug, Clone)]
pub enum Literal{
    Integer(i32),
    String(String),
    Float(f32),
    Bool(bool),
    Null
}

impl Literal{
    pub fn stringify(literal: Literal) -> String{
        match literal{
            Literal::Null => String::from("nil"),
            Literal::String(value) => value.to_string(),
            Literal::Float(value) => value.to_string(),
            Literal::Bool(value) => value.to_string(),
            Literal::Integer(value) => value.to_string()
        }
    }
}