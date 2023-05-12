#[derive(Debug, Clone)]
pub enum Literal{
    Integer(i32),
    String(String),
    Float(f32),
    Bool(bool),
    Null
}