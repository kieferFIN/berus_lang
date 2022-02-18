use std::fmt::{Display, Formatter};
use crate::ast::{ConstantValue, FunctionDef, TypeName, VariableDef};

#[derive(Debug)]
pub struct Expr {
    pub exprs: Vec<PartialExpr>,
    pub operands: Vec<Operand>,
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (expr, op) in self.exprs.iter().zip(&self.operands) {
            write!(f, " {:?} {:?}", expr, op)?
        };
        if let Some(e) = self.exprs.last() {
            write!(f, " {:?};", e)?
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum Operand {
    //TODO: more operands
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum PartialExpr {
    Block(BlockExpr),
    If(IfExpr),
    FunctionCall(FunctionCallExpr),
    Variable(VariableExpr),
    Lambda(FunctionDef),
}

#[derive(Debug)]
pub struct BlockExpr {}

#[derive(Debug)]
pub struct IfExpr {
    pub cond_expr: Expr,
    pub main_branch: Expr,
    pub else_branch: Option<Expr>,
}

#[derive(Debug)]
pub struct FunctionCallExpr {
    pub name: String,
    pub params: Vec<Expr>,
}

#[derive(Debug)]
pub enum VariableExpr {
    Variable(String),
    Constant(ConstantValue),
}