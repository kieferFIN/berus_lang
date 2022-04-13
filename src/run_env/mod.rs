use std::collections::HashMap;
use std::ops::Deref;

use nom::Finish;

use crate::ast::{Module, VariableDef};
use crate::ast::expr::{Expr, FunctionCallExpr, FunctionDef, IfExpr, Operand, PartialExpr, VariableExpr};
use crate::ast::states::{Unverified, Verified};
use crate::ast::types::VariableType;
use crate::ast::variable::{ConstantValue, VariableName};
use crate::parser::expr::parse_expr;
use crate::parser::parse_module;
pub use crate::run_env::data_obj::{DataObj, RefDataObj};
use crate::run_env::variable_stack::VariableStack;
use crate::std_library::{BoolType, FuncType, IntType, StdMod};
use crate::verify::variable_mng::VariableManager;

pub mod variable_stack;
pub mod data_obj;

pub struct RunEnv<S: StdMod> {
    modules: HashMap<String, Module<Verified>>,
    variable_stack: VariableStack<S>,
}

impl<S: StdMod> RunEnv<S> {
    pub fn new() -> Self {
        Self { modules: HashMap::new(), variable_stack: VariableStack::new() }
    }

    pub fn add_module(&mut self, name: String, module: Module<Unverified>) -> Result<(), String> {
        let m = module.verify()?;
        self.variable_stack.extend(m.run())?;
        self.modules.insert(name, m);

        Ok(())
    }

    pub fn parse_and_add(&mut self, name: String, code: &str) -> Result<(), String> {
        let (unparsed, module) = parse_module(code).finish().map_err(|e| format!("{:?}", e))?;
        if unparsed.len() > 0 {
            return Err(format!("unparsed: {}", unparsed));
        } else { self.add_module(name, module) }
    }

    pub fn print(&self) {
        println!("nmb of mod:{}", self.modules.len());
        for (n, m) in &self.modules {
            println!("{}:{}", n, m)
        }
    }

    pub fn find_variable(&self, name: &str) -> Option<RefDataObj<S>> {
        self.variable_stack.try_find_variable(name)
    }

    pub fn run(&self, expr: &str) -> Result<(VariableType, RefDataObj<S>), String> {
        let (_, e) = parse_expr(expr).map_err(|e| e.to_string())?;
        //println!("{}",e);
        let mut v_mng = VariableManager::new();
        for (_, m) in &self.modules {
            v_mng.add_module(m)
        }
        let t = e.check_type(&mut v_mng)?;
        let r = e.run(&self.variable_stack);
        Ok((t, r))
    }
}

impl Module<Verified> {
    fn run<S: StdMod>(&self) -> VariableStack<S> {
        let mut known_variables = VariableStack::new();
        for VariableName { name, variable: VariableDef { value: expr, v_type: _ } } in &self.variables {
            let r: RefDataObj<S> = expr.run(&known_variables);
            known_variables.add_variable(name.clone(), r);
        }
        known_variables
    }
}

fn calculate<S: StdMod>(lhs: &DataObj<S>, o: &Operand, rhs: &DataObj<S>) -> DataObj<S> {
    match (lhs, o, rhs) {
        (DataObj::Int(l), Operand::Plus, DataObj::Int(r)) => S::int_obj(l.plus(r)),
        (DataObj::Int(l), Operand::Minus, DataObj::Int(r)) => S::int_obj(l.minus(r)),
        (DataObj::Int(l), Operand::Lt, DataObj::Int(r)) => S::bool_obj(l.lt(r)),
        (_, _, _) => panic!("Runtime ERROR: Can not calculate {} {:?} {}", lhs.type_str(), o, rhs.type_str())
    }
}

impl Expr {
    pub(crate) fn run<S: StdMod>(&self, known_variables: &VariableStack<S>) -> RefDataObj<S> {
        if self.operands.len() == 0 {
            self.exprs.first().unwrap().run(known_variables)
        } else {
            let mut it = self.exprs.iter();
            let mut value = it.next().unwrap().run(known_variables).borrow().deref().clone();
            for (e, o) in it.zip(&self.operands) {
                value = calculate(&value, o, &e.run(known_variables).borrow().deref().clone());
            }
            value.into_ref()
        }
    }
}

impl PartialExpr {
    fn run<S: StdMod>(&self, known_variables: &VariableStack<S>) -> RefDataObj<S> {
        match self {
            PartialExpr::Block(_) => todo!(),
            PartialExpr::If(v) => v.run(known_variables),
            PartialExpr::FunctionCall(v) => v.run(known_variables),
            PartialExpr::Variable(v) => v.run(known_variables),
            PartialExpr::Lambda(v) => v.run(known_variables),
            PartialExpr::Tuple(_) => todo!(),
        }
    }
}

impl VariableExpr {
    fn run<S: StdMod>(&self, known_variables: &VariableStack<S>) -> RefDataObj<S> {
        match self {
            VariableExpr::Variable(name) => known_variables.find_variable(name),
            VariableExpr::Constant(v) => match v {
                ConstantValue::Integer(v) => S::int_create(*v),
                ConstantValue::Float(v) => S::float_create(*v),
                ConstantValue::String(v) => S::string_create(v.clone())
            }.into_ref()
        }
    }
}

impl FunctionDef {
    fn run<S: StdMod>(&self, known_variables: &VariableStack<S>) -> RefDataObj<S> {
        let params = self.parameters.iter().map(|(name, _)| name.clone()).collect();

        let mut closure = HashMap::new();
        for (name, _) in &self.closure {
            let v = known_variables.find_variable(name);
            closure.insert(name.clone(), v);
        };
        S::func_create(self.expr.clone(), params, closure).into_ref()
    }
}

impl FunctionCallExpr {
    fn run<S: StdMod>(&self, known_variables: &VariableStack<S>) -> RefDataObj<S> {
        let ref_obj = known_variables.find_variable(&self.name);
        let mut closure_variables = VariableStack::new();
        closure_variables.add_variable("self_fn".to_string(), ref_obj.clone());

        let obj = ref_obj.borrow();
        match obj.deref() {
            DataObj::Func(f) => {
                let p = self.params.iter().map(|e| e.run(known_variables)).collect();
                f.call(p)
            }
            _ => panic!("Runtime ERROR: Can not call function {}.", self.name)
        }
    }
}

impl IfExpr {
    fn run<S: StdMod>(&self, known_variables: &VariableStack<S>) -> RefDataObj<S> {
        match self.cond_expr.run(known_variables).borrow().deref() {
            DataObj::Bool(cond) => {
                if cond.is_true() {
                    self.main_branch.run(known_variables)
                } else {
                    match &self.else_branch {
                        None => S::empty_create().into_ref(),
                        Some(e) => e.run(known_variables)
                    }
                }
            }
            _ => panic!("runtime ERROR: can not get bool")
        }
    }
}