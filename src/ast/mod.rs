use std::collections::{ HashSet};
use std::fmt::{Debug};

use crate::ast::expr::Expr;
use crate::ast::states::AstState;
use crate::ast::structs::StructDef;
use crate::ast::types::{ VariableType};

pub mod expr;
pub mod types;
pub mod states;
pub mod displays;
pub mod structs;
mod utils;

pub struct Module< S: AstState> {
    pub variables: Vec<VariableName>,
    pub structs: HashSet<StructDef>,
    pub _state: std::marker::PhantomData<S>,
}

/*impl<'s> FromStr for Module<'s, Unverified> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (unparsed, module) = parse_module(s).finish().map_err(|e| format!("{:?}", e))?;
        if unparsed.len() > 0 {
            Err(format!("unparsed: {}", unparsed))
        } else { Ok(module) }
    }
}*/

/*impl FromStr for Module<Verified> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m: Module<Unverified> = s.parse()?;
        m.verify()
    }
}*/

pub struct VariableDef {
    pub value: Expr,
    pub v_type: VariableType,
}

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