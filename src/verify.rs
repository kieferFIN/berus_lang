use std::collections::HashMap;
use std::fmt::format;
use crate::ast::{funcdef_from_type_name, FunctionDef, Module, TypeName, Unverified, VariableName, Verified};
use crate::ast::expr::{BlockExpr, Expr, FunctionCallExpr, IfExpr, Operand, PartialExpr, VariableExpr};

impl Module<Unverified> {
    pub fn verify(mut self) -> Result<Module<Verified>, String> {
        let mut variable_mng = VariableManager::new();
        for VariableName { name, variable } in &mut self.variables {
            let checked_type = variable.value.check_type(&mut variable_mng, &variable.type_info)?;
            variable_mng.add_variable(name.to_owned(), checked_type.to_string(), variable.mutable);
            variable.type_info = Some(checked_type);
        }
        Ok(Module { variables: self.variables, _state: Default::default() })
    }
}

impl Expr {
    fn check_type(&self, variable_mng: &mut VariableManager, expected_type: &Option<TypeName>) -> Result<TypeName, String> {
        let mut expr_iter = self.exprs.iter();
        let first_expr = expr_iter.next().unwrap();
        let mut type_name = first_expr.check_type(variable_mng, &None)?;

        for (e, o) in expr_iter.zip(&self.operands) {
            type_name = check_valid_operand(&type_name, o, &e.check_type(variable_mng, &None)?)
        }

        is_correct_type(expected_type, type_name)
    }
}

fn check_valid_operand(lhs: &TypeName, op: &Operand, rhs: &TypeName) -> TypeName {
    //TODO: do valid, this is mockup
    lhs.to_owned()
}

impl PartialExpr {
    fn check_type(&self, variable_mng: &mut VariableManager, expected_type: &Option<TypeName>) -> Result<TypeName, String> {
        match self {
            PartialExpr::Block(e) => e.check_type(variable_mng, expected_type),
            PartialExpr::If(e) => e.check_type(variable_mng, expected_type),
            PartialExpr::Variable(e) => e.check_type(variable_mng, expected_type),
            PartialExpr::FunctionCall(e) => e.check_type(variable_mng, expected_type),
            PartialExpr::Lambda(e) => e.check_type(variable_mng, expected_type),
        }
    }
}

impl BlockExpr {
    fn check_type(&self, variable_mng: &mut VariableManager, expected_type: &Option<TypeName>) -> Result<TypeName, String> {
        Err("Unimplemented".to_string())
    }
}

impl IfExpr {
    fn check_type(&self, variable_mng: &mut VariableManager, expected_type: &Option<TypeName>) -> Result<TypeName, String> {
        Err("If Expr checking is unimplemented".to_string())
    }
}

impl FunctionCallExpr {
    fn check_type(&self, variable_mng: &mut VariableManager, expected_type: &Option<TypeName>) -> Result<TypeName, String> {
        match variable_mng.check_variable(&self.name, &false){
            None => Err(format!("Cannot find variable: {}", &self.name)),
            Some(tn) => {
                match funcdef_from_type_name(&tn){
                    None => Err(format!("Cannot find function: {}", &self.name)),
                    Some((params, ret)) => {
                        if params.len() != self.params.len() {
                            Err(format!("Wrong number of params. expected:{}, found:{}",params.len(), self.params.len()))
                        }else {
                            for ((tn,_m),e ) in params.iter().zip(&self.params){
                                e.check_type(variable_mng,&Some(tn.to_owned()))?;
                            }
                            Ok(ret)
                        }
                    }
                }
            }
        }
    }
}

impl VariableExpr {
    fn check_type(&self, variable_mng: &mut VariableManager, expected_type: &Option<TypeName>) -> Result<TypeName, String> {
        let tn = match self {
            VariableExpr::Variable(name) => variable_mng.check_variable(name, &false).unwrap_or(
                format!("Cannot find variable: {}", name)
            ),
            VariableExpr::Constant(cv) => cv.to_type_info()
        };
        is_correct_type(expected_type,tn)

    }
}

impl FunctionDef {
    fn check_type(&self, variable_mng: &mut VariableManager, expected_type: &Option<TypeName>) -> Result<TypeName, String> {
        let mut local_variables = VariableManager::new();
        for (name,mutable) in &self.closure{
            match variable_mng.check_variable(name,mutable) {
                Some(type_name) => local_variables.add_variable(name.to_owned(),type_name.to_owned(),*mutable),
                None => return Err(format!("Cannot find variable: {}", name))
            }
        };
        for (name, type_name, mutable) in &self.parameters{
            local_variables.add_variable(name.to_owned(), type_name.to_owned(),*mutable)
        }
        local_variables.add_variable("self".to_string(),self.type_name(),false);
        self.expr.check_type(&mut local_variables,&Some(self.return_type.to_owned()))?;
        is_correct_type(expected_type,self.type_name())

    }
}

struct VariableManager {
    variables: Vec<HashMap<String, (TypeName, bool)>>,
}

impl VariableManager {
    fn new() -> Self {
        VariableManager {
            variables: vec![HashMap::new()]
        }
    }

    fn add_variable(&mut self, name: String, type_name: TypeName, mutable: bool) {
        self.variables.last_mut().unwrap().insert(name, (type_name, mutable));
    }
    fn check_variable(&self, name: &String, mutable: &bool) -> Option<TypeName> {
        for layer in self.variables.iter().rev() {
            match layer.get(name) {
                Some((ty, mu)) => if !mutable || *mu { return Some(ty.to_owned()); },
                _ => ()
            };
        };
        None
    }

    fn add_layer(&mut self) {
        self.variables.push(HashMap::new())
    }

    fn pop_layer(&mut self) {
        self.variables.pop();
        if self.variables.is_empty() {
            self.add_layer()
        }
    }
}

fn is_correct_type(expected: &Option<TypeName>, correct:TypeName)->Result<TypeName, String>{
    match expected {
        None => Ok(correct),
        Some(et) => {
            if et == &correct {
                Ok(correct)
            }else { Err(format!("Wrong type. expected:{}, found:{}",et,correct)) }
        }
    }
}