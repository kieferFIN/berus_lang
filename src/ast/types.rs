use std::fmt;
use std::fmt::{Display, Formatter};

use crate::ast::expr::FunctionDef;
use crate::ast::utils::str_from_iter;

#[derive(PartialEq, Eq, Clone)]
pub enum TypeInfo {
    Struct(String),
    Tuple(Vec<TypeInfo>),
    Function(FuncType),
    Unknown,
}

impl TypeInfo {
    pub fn empty() -> Self {
        TypeInfo::Tuple(Vec::new())
    }
}

impl Display for TypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeInfo::Struct(name) => write!(f, "{}", name),
            TypeInfo::Tuple(v) => write!(f, "({})", str_from_iter(v.iter(), ",")),
            TypeInfo::Function(func) => write!(f, "<{}>:{}", str_from_iter(func.params.iter(), ","), func.return_type),
            TypeInfo::Unknown => write!(f, "#UNKNOWN")
        }
    }
}

impl From<&FunctionDef> for FuncType {
    fn from(fd: &FunctionDef) -> Self {
        let params = fd.parameters.iter().map(|(_, t)| t.clone()).collect();
        FuncType { params, return_type: Box::new(fd.return_type.clone()) }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FuncType {
    pub(crate) params: Vec<VariableType>,
    pub(crate) return_type: Box<VariableType>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct VariableType {
    pub mutable: bool,
    pub info: TypeInfo,
}

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

/*impl<'s> VariableType<'s>{
    pub(crate) fn from_str(st: &'s str) -> Result<Self, String>{
        parse_variable_type(st).finish().map(|(_, t)| t).map_err(|e| e.to_string())
    }
}*/