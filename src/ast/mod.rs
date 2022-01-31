use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

pub struct Module {
    pub constants: HashMap<String, ConstantValue>,
    pub functions: HashMap<String, FunctionDef>,
}

#[derive(Debug)]
pub enum TypeInfo {
    Integer,
    Float,
    Struct(String),
}

impl From<String> for TypeInfo {
    fn from(type_name: String) -> Self {
        match type_name.as_str() {
            "int" => TypeInfo::Integer,
            "float" => TypeInfo::Float,
            name => TypeInfo::Struct(name.to_owned())
        }
    }
}

#[derive(Debug)]
pub enum ConstantValue {
    Integer(i32),
    Float(f32),
}

#[derive(Debug)]
pub struct FunctionDef {
    pub parameters: Vec<(String, TypeInfo)>,
    pub return_type: Option<TypeInfo>,
}

/*pub struct Constant{
    pub name: String,
    pub value: ConstantValue
}

impl PartialEq<Self> for Constant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Constant {}

impl Hash for Constant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}*/
