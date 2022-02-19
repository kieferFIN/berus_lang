use std::fmt;
use std::fmt::{Display, Formatter};

use crate::ast::ConstantValue;
use crate::ast::types::VariableType;
use crate::ast::utils::str_from_iter;

pub struct Expr {
    pub exprs: Vec<PartialExpr>,
    pub operands: Vec<Operand>,
}

#[derive(Debug)]
pub enum Operand {
    //TODO: more operands
    Plus,
    Minus,
    Lt,
}

pub enum PartialExpr {
    Block(BlockExpr),
    If(IfExpr),
    FunctionCall(FunctionCallExpr),
    Variable(VariableExpr),
    Lambda(FunctionDef),
    Tuple(TupleDef),
}

pub struct BlockExpr {}

pub struct IfExpr {
    pub cond_expr: Expr,
    pub main_branch: Expr,
    pub else_branch: Option<Expr>,
}

pub struct FunctionCallExpr {
    pub name: String,
    pub params: Vec<Expr>,
}

pub enum VariableExpr {
    Variable(String),
    Constant(ConstantValue),
}

pub struct FunctionDef {
    pub parameters: Vec<(String, VariableType)>,
    pub closure: Vec<(String, bool)>,
    pub return_type: VariableType,
    pub expr: Expr,
}

pub struct TupleDef {
    pub items: Vec<Expr>,
}