use std::collections::HashSet;

use crate::ast::expr::Expr;
use crate::ast::states::AstState;
use crate::ast::structs::StructDef;
use crate::ast::types::VariableType;
use crate::ast::variable::VariableName;

pub mod expr;
pub mod types;
pub mod states;
pub mod displays;
pub mod structs;
mod utils;
pub mod variable;

pub struct Module<S: AstState> {
    pub variables: Vec<VariableName>,
    pub structs: HashSet<StructDef>,
    pub _state: std::marker::PhantomData<S>,
}

pub struct VariableDef {
    pub value: Expr,
    pub v_type: VariableType,
}
