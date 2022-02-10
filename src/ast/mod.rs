pub mod expr;

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use nom::Finish;
use crate::ast::expr::{Expr, PartialExpr};
use crate::parser::parse_module;

#[derive(Debug)]
pub struct Module {
    pub variables: Vec<VariableName>,
}

impl FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (unparsed, module) = parse_module(s).finish().map_err(|e| format!("{:?}", e))?;
        if unparsed.len() > 0 {
            Err(format!("unparsed: {}", unparsed))
        } else { Ok(module) }
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for vd in &self.variables {
            writeln!(f, "{} = {}", vd.name, vd.variable)?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum TypeInfo {
    Integer,
    Float,
    String,
    Struct(String),
    Enum(String),
    Tuple(Vec<TypeInfo>),
    Function(Vec<TypeInfo>, String),
    None,
}

impl From<String> for TypeInfo {
    fn from(type_name: String) -> Self {
        match type_name.as_str() {
            "int" => TypeInfo::Integer,
            "float" => TypeInfo::Float,
            "str" => TypeInfo::String,
            name => TypeInfo::Struct(name.to_owned())
        }
    }
}

#[derive(Debug)]
pub struct VariableDef {
    pub mutable: bool,
    pub value: Expr,
    pub type_info: Option<TypeInfo>,
}

impl Display for VariableDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let m = if self.mutable { "mut" } else { "" };
        let ti = self.type_info.as_ref().unwrap_or(&TypeInfo::None);
        write!(f, "{} {:?} : {}", m, ti, self.value)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConstantValue {
    Integer(i32),
    Float(f32),
    String(String),
}

#[derive(Debug)]
pub struct FunctionDef {
    pub parameters: Vec<(String, TypeInfo)>,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct VariableName {
    pub name: String,
    pub variable: VariableDef,
}
