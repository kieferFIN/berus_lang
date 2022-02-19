use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use nom::Finish;

use crate::ast::ConstantValue;
use crate::ast::expr::FunctionDef;
use crate::ast::utils::str_from_iter;
use crate::parser::parse_variable_type;

#[derive(PartialEq, Clone)]
pub enum TypeInfo {
    //Integer,
    //Float,
    //String,
    Struct(String),
    //Enum(String),
    Tuple(Vec<TypeInfo>),
    Function(Vec<VariableType>, String),
    Unknown,
}

impl TypeInfo {
    pub fn empty() ->Self{
        TypeInfo::Tuple(Vec::new())
    }
}

impl Display for TypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeInfo::Struct(name) => write!(f, "{}", name),
            TypeInfo::Tuple(v) => write!(f, "({})", str_from_iter(v.iter(), ",")),
            TypeInfo::Function(p, r) => write!(f, "<{}>:{}", str_from_iter(p.iter(), ","), r),
            TypeInfo::Unknown => write!(f, "#UNKNOWN")
        }
    }
}

impl From<&ConstantValue> for TypeInfo {
    fn from(c: &ConstantValue) -> Self {
        TypeInfo::Struct(match c {
            ConstantValue::Integer(_) => "Int",
            ConstantValue::Float(_) => "Float",
            ConstantValue::String(_) => "String"
        }.to_string())
    }
}

impl From<&FunctionDef> for TypeInfo {
    fn from(fd: &FunctionDef) -> Self {
        let params = fd.parameters.iter().map(|(_, t)| t.clone()).collect();
        TypeInfo::Function(params, fd.return_type.to_string())
    }
}

#[derive(Clone, PartialEq)]
pub struct VariableType {
    pub mutable: bool,
    pub info: TypeInfo,
}


impl Display for VariableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.mutable { "mut " } else { "" }, self.info)
    }
}

/*impl Default for VariableType {
    fn default() -> Self {
        VariableType{ mutable: f, info: TypeInfo::Unknown }
    }
}*/

impl VariableType {
    pub fn check_expected(self, expected: &VariableType) -> Result<VariableType, String> {
        if expected.info == TypeInfo::Unknown {
            if self.check_mutability(expected.mutable) {
                Ok(VariableType { mutable: expected.mutable, info: self.info })
            } else { Err("Incorrect mutability.".to_string()) }
        } else if expected.info != self.info {
            Err(format!("Wrong type. expected:{}, found:{}", expected.info, self.info))
        } else if self.check_mutability(expected.mutable) {
            Ok(VariableType { mutable: expected.mutable, info: self.info })
        } else {
            Err("Incorrect mutability.".to_string())
        }
    }

    pub fn check_mutability(&self, expected_mutability: bool) -> bool {
        self.mutable || !expected_mutability
    }
}

impl FromStr for VariableType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_variable_type(s).finish().map(|(_, t)| t).map_err(|e| e.to_string())
    }
}