use std::fmt;
use std::fmt::{Display, Formatter};

use crate::ast::{Module, VariableDef};
use crate::ast::expr::{Expr, FunctionCallExpr, FunctionDef, IfExpr, PartialExpr, TupleDef, VariableExpr};
use crate::ast::states::AstState;
use crate::ast::utils::str_from_iter;

impl<S: AstState> Display for Module<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for vd in &self.variables {
            writeln!(f, "{}{}", vd.name, vd.variable)?
        }
        Ok(())
    }
}

impl Display for VariableDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}= {}", self.v_type, self.value)?;
        Ok(())
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (expr, op) in self.exprs.iter().zip(&self.operands) {
            write!(f, "{} {:?} ", expr, op)?
        };
        if let Some(e) = self.exprs.last() {
            write!(f, "{};", e)?
        };
        Ok(())
    }
}

impl Display for PartialExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PartialExpr::Block(_) => write!(f, "BLOCK"),
            PartialExpr::If(e) => write!(f, "{}", e),
            PartialExpr::FunctionCall(e) => write!(f, "{}", e),
            PartialExpr::Variable(e) => write!(f, "{}", e),
            PartialExpr::Lambda(e) => write!(f, "{}", e),
            PartialExpr::Tuple(e) => write!(f, "{}",e)
        }
    }
}

impl Display for IfExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "IF({}){}", self.cond_expr, self.main_branch)?;
        match &self.else_branch {
            Some(e) => write!(f, " else {}", e),
            None => Ok(())
        }
    }
}

impl Display for FunctionCallExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, str_from_iter(self.params.iter(), ","))
    }
}

impl Display for VariableExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VariableExpr::Variable(name) => write!(f, "{}", name),
            VariableExpr::Constant(cv) => write!(f, "{:?}", cv)
        }
    }
}

impl Display for FunctionDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}><{}>:{}->{}",
               str_from_iter(self.parameters.iter().map(|(name, vt)| format!("{}:{}", name, vt)), ","),
               str_from_iter(self.closure.iter().map(|(name, m)| format!("{}{}", if *m { "mut " } else { "" }, name)), ","),
               self.return_type,
               self.expr)
    }
}

impl Display for TupleDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"({})",str_from_iter(self.items.iter(),","))
    }
}