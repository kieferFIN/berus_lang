use crate::ast::VariableDef;

#[derive(Debug, Clone)]
pub enum ConstantValue {
    Integer(i32),
    Float(f32),
    String(String),
}

pub struct VariableName {
    pub name: String,
    pub variable: VariableDef,
}