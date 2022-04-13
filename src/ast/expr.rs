use crate::ast::ConstantValue;
use crate::ast::types::VariableType;

#[derive(Clone)]
pub struct Expr {
    pub exprs: Vec<PartialExpr>,
    pub operands: Vec<Operand>,
}

#[derive(Debug,Clone)]
pub enum Operand {
    //TODO: more operands
    Plus,
    Minus,
    Lt,
}

#[derive(Clone)]
pub enum PartialExpr {
    Block(BlockExpr),
    If(IfExpr),
    FunctionCall(FunctionCallExpr),
    Variable(VariableExpr),
    Lambda(FunctionDef),
    Tuple(TupleDef),
}

#[derive(Clone)]
pub struct BlockExpr {}

#[derive(Clone)]
pub struct IfExpr {
    pub cond_expr: Expr,
    pub main_branch: Expr,
    pub else_branch: Option<Expr>,
}

#[derive(Clone)]
pub struct FunctionCallExpr {
    pub name: String,
    pub params: Vec<Expr>,
}

#[derive(Clone)]
pub enum VariableExpr {
    Variable(String),
    Constant(ConstantValue),
}

#[derive(Clone)]
pub struct FunctionDef{
    pub parameters: Vec<(String, VariableType)>,
    pub closure: Vec<(String, bool)>,
    pub return_type: VariableType,
    pub expr: Expr,
}

#[derive(Clone)]
pub struct TupleDef {
    pub items: Vec<Expr>,
}