use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use nom::Finish;

use crate::ast::expr::Expr;
use crate::ast::states::{AstState, Unverified, Verified};
use crate::ast::types::VariableType;
use crate::parser::parse_module;

pub mod expr;
pub mod types;
pub mod states;
mod utils;
pub mod displays;

pub struct Module<S: AstState> {
    pub variables: Vec<VariableName>,
    pub _state: std::marker::PhantomData<S>,
}

impl FromStr for Module<Unverified> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (unparsed, module) = parse_module(s).finish().map_err(|e| format!("{:?}", e))?;
        if unparsed.len() > 0 {
            Err(format!("unparsed: {}", unparsed))
        } else { Ok(module) }
    }
}

impl FromStr for Module<Verified> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m: Module<Unverified> = s.parse()?;
        m.verify()
    }
}

pub struct VariableDef {
    pub value: Expr,
    pub v_type: VariableType,
}

#[derive(Debug)]
pub enum ConstantValue {
    Integer(i32),
    Float(f32),
    String(String),
}

pub struct VariableName {
    pub name: String,
    pub variable: VariableDef,
}