use crate::ast::{Module, VariableName};
use crate::ast::expr::{BlockExpr, Expr, FunctionCallExpr, FunctionDef, IfExpr, Operand, PartialExpr, TupleDef, VariableExpr};
use crate::ast::states::{Unverified, Verified};
use crate::ast::types::{TypeInfo, VariableType};
use crate::verify::variable_mng::VariableManager;

mod variable_mng;

impl Module<Unverified> {
    pub fn verify(mut self) -> Result<Module<Verified>, String> {
        let mut variable_mng = VariableManager::new();
        for VariableName { name, variable } in &mut self.variables {
            let checked_type = variable.value.check_type(&mut variable_mng)?.check_expected(&variable.v_type)?;
            //println!("{}: {}, {}",name,variable.v_type,checked_type);

            variable_mng.add_variable(name.to_owned(), checked_type.clone());
            variable.v_type = checked_type;
        }
        Ok(Module { variables: self.variables, _state: Default::default() })
    }
}

impl Expr {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String> {
        let mut expr_iter = self.exprs.iter();
        let first_expr = expr_iter.next().unwrap();
        let mut type_name = first_expr.check_type(variable_mng)?;

        for (e, o) in expr_iter.zip(&self.operands) {
            type_name = check_valid_operand(&type_name, o, &e.check_type(variable_mng)?)
        }

        Ok(type_name)
    }
}

fn check_valid_operand(lhs: &VariableType, op: &Operand, rhs: &VariableType) -> VariableType {
    //TODO: do valid, this is mockup
    match op {
        Operand::Lt => VariableType{ mutable: true, info: TypeInfo::Struct("Bool".to_string()) },
        Operand::Plus | Operand::Minus => (*lhs).to_owned()
    }

}

impl PartialExpr {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String> {
        match self {
            PartialExpr::Block(e) => e.check_type(variable_mng),
            PartialExpr::If(e) => e.check_type(variable_mng),
            PartialExpr::Variable(e) => e.check_type(variable_mng),
            PartialExpr::FunctionCall(e) => e.check_type(variable_mng),
            PartialExpr::Lambda(e) => e.check_type(variable_mng),
            PartialExpr::Tuple(e) => e.check_type(variable_mng)
        }
    }
}

impl BlockExpr {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String> {
        Err("Unimplemented".to_string())
    }
}

impl IfExpr {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String> {
        let cond_type = self.cond_expr.check_type(variable_mng)?;
        if cond_type.info == TypeInfo::Struct("Bool".to_string()) {
            let branch_type = self.main_branch.check_type(variable_mng)?;
            match &self.else_branch {
                None => {
                    if branch_type.info != TypeInfo::empty() {
                        Err(format!("If without else, must return empty. found:{}", branch_type))
                    } else { Ok(branch_type) }
                }
                Some(eb) => {
                    let else_type = eb.check_type(variable_mng)?;
                    if else_type.info != branch_type.info {
                        Err(format!("Both if branches must return same type. found: main:{}, else:{}", branch_type, else_type))
                    } else {
                        Ok(VariableType { mutable: else_type.mutable && branch_type.mutable, info: branch_type.info })
                    }
                }
            }
        } else { Err(format!("Conditional expression must retunr Bool. found:{}", cond_type)) }
    }
}

impl FunctionCallExpr {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String> {
        match variable_mng.find_variable(&self.name) {
            None => Err(format!("Cannot find variable: {}", &self.name)),
            Some(VariableType { mutable: _, info: TypeInfo::Function(params, ret) }) => {
                if params.len() != self.params.len() {
                    Err(format!("Wrong number of params. expected:{}, found:{}", params.len(), self.params.len()))
                } else {
                    for (vt, e) in params.iter().zip(&self.params) {
                        e.check_type(variable_mng)?.check_expected(vt)?;
                    }
                    Ok((&ret).parse()?)
                }
            }
            Some(t) => Err(format!("Variable {} is not function: found type:{}", &self.name, t)),
        }
    }
}

impl VariableExpr {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String> {
        match self {
            VariableExpr::Variable(name) => variable_mng.find_variable(name).ok_or(
                format!("Cannot find variable: {}", name)
            ),
            VariableExpr::Constant(cv) => Ok(VariableType { mutable: true, info: cv.into() })
        }
    }
}

impl FunctionDef {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String> {
        let mut local_variables = VariableManager::new();
        for (name, mutable) in &self.closure {
            match variable_mng.find_variable(name) {
                Some(vt) => {
                    if vt.check_mutability(*mutable) {
                        local_variables.add_variable(name.to_owned(), VariableType { mutable: *mutable, info: vt.info })
                    }
                }
                None => return Err(format!("Cannot find variable: {}", name))
            }
        };
        for (name, param_type) in &self.parameters {
            local_variables.add_variable(name.to_owned(), (*param_type).clone())
        }
        local_variables.add_variable("self".to_string(), VariableType { mutable: false, info: self.into() });
        //println!("{:?}",local_variables);
        let ret_type = self.expr.check_type(&mut local_variables)?;
        ret_type.check_expected(&self.return_type)?;
        Ok(VariableType { mutable: true, info: self.into() })
    }
}

impl TupleDef {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String>{
        let types_r:Result<Vec<_>,_> = self.items.iter().map(|e|e.check_type(variable_mng).map(|t|t.info)).collect();
        let types = types_r?;
        Ok(VariableType{ mutable: false, info: TypeInfo::Tuple(types) })

    }
}

