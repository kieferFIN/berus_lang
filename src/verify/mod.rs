use crate::ast::{ConstantValue, Module, VariableName};
use crate::ast::expr::{BlockExpr, Expr, FunctionCallExpr, FunctionDef, IfExpr, Operand, PartialExpr, TupleDef, VariableExpr};
use crate::ast::states::{Unverified, Verified};
use crate::ast::types::{FuncType, TypeInfo, VariableType};
use crate::verify::variable_mng::VariableManager;

pub(crate) mod variable_mng;

const INT_TYPE: &'static str = "Int";
const FLOAT_TYPE: &'static str = "Float";
const STRING_TYPE: &'static str = "String";
const BOOL_TYPE: &'static str = "Bool";

impl Module< Unverified> {
    pub fn verify(mut self) -> Result<Module<Verified>, String> {
        let mut variable_mng = VariableManager::new();
        for VariableName { name, variable } in &mut self.variables {
            let checked_type = variable.value.check_type(&mut variable_mng)?.check_expected(&variable.v_type)?;
            //println!("{}: {}, {}",name,variable.v_type,checked_type);

            variable_mng.add_variable(name.to_string(), checked_type.clone());
            variable.v_type = checked_type;
        }
        Ok(Module { variables: self.variables, structs: self.structs, _state: Default::default() })
    }
}

impl Expr {
    pub(crate) fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String> {
        let mut expr_iter = self.exprs.iter();
        let first_expr = expr_iter.next().unwrap();
        let mut type_name = first_expr.check_type(variable_mng)?;

        for (e, o) in expr_iter.zip(&self.operands) {
            type_name = check_valid_operand(&type_name, o, &e.check_type(variable_mng)?)?
        }

        Ok(type_name)
    }
}

fn check_valid_operand(lhs: &VariableType, op: &Operand, rhs: &VariableType) -> Result<VariableType,String> {
    match (&lhs.info, op, &rhs.info) {
        (TypeInfo::Struct(l), Operand::Plus, TypeInfo::Struct(r)) if l==r && (l==INT_TYPE || l ==FLOAT_TYPE) => Ok(VariableType{mutable:true, info:lhs.info.clone()}),
        (TypeInfo::Struct(l), Operand::Minus, TypeInfo::Struct(r)) if l==r && (l==INT_TYPE || l ==FLOAT_TYPE) => Ok(VariableType{mutable:true, info:lhs.info.clone()}),
        (TypeInfo::Struct(l), Operand::Lt, TypeInfo::Struct(r)) if l==r && (l==INT_TYPE || l ==FLOAT_TYPE) => Ok(VariableType{mutable:true, info:TypeInfo::Struct(BOOL_TYPE.to_string())}),
        _ => Result::Err(format!("not defined: {} {:?} {}", lhs, op, rhs))
    }
    //TODO: check operands for local types

    //Err(format!("not defined: {} {:?} {}", lhs, op, rhs))


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
        if cond_type.info == get_condition_type() {
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
        } else { Err(format!("Conditional expression must return {}. found:{}",get_condition_type(), cond_type)) }
    }
}

impl FunctionCallExpr {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String> {
        match variable_mng.find_variable(&self.name) {
            None => Err(format!("Cannot find variable: {}", &self.name)),
            Some(VariableType { mutable: _, info: TypeInfo::Function(FuncType{params, return_type: ret}) }) => {
                if params.len() != self.params.len() {
                    Err(format!("Wrong number of params. expected:{}, found:{}", params.len(), self.params.len()))
                } else {
                    for (vt, e) in params.iter().zip(&self.params) {
                        e.check_type(variable_mng)?.check_expected(vt)?;
                    }
                    Ok(*ret)
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
            VariableExpr::Constant(cv) => Ok(VariableType { mutable: true, info: get_type(cv) })
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
        local_variables.add_variable("self_fn".to_string(), VariableType { mutable: false, info: TypeInfo::Function(self.into()) });
        //println!("{:?}",local_variables);
        let ret_type = self.expr.check_type(&mut local_variables)?;
        ret_type.check_expected(&self.return_type)?;
        Ok(VariableType { mutable: true, info: TypeInfo::Function(self.into()) })
    }
}

impl TupleDef {
    fn check_type(&self, variable_mng: &mut VariableManager) -> Result<VariableType, String>{
        let mut types = Vec::new();
        for e in &self.items{
            let t = e.check_type(variable_mng)?;
            types.push(t.info);
        }
/*        let types_r:Result<Vec<_>,_> = self.items.iter().map(|e|e.check_type(variable_mng, std_mod).map(|t|t.info)).collect();
        let types = types_r?;*/
        Ok(VariableType{ mutable: false, info: TypeInfo::Tuple(types) })

    }
}

fn get_type(c: &ConstantValue) -> TypeInfo {
    TypeInfo::Struct(match c {
        ConstantValue::Integer(_) => INT_TYPE,
        ConstantValue::Float(_) => FLOAT_TYPE,
        ConstantValue::String(_) => STRING_TYPE
    }.to_string())
}

fn get_condition_type() -> TypeInfo {
    TypeInfo::Struct(BOOL_TYPE.to_string())
}

