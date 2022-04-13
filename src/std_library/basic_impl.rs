use std::collections::HashMap;
use crate::ast::expr::Expr;
use crate::run_env::{ RefDataObj};
use crate::run_env::variable_stack::VariableStack;
use crate::std_library::{BoolType, EmptyType, FloatType, FuncType, IntType, StdMod, StringType};

#[derive(Clone)]
pub struct StdModBasic{}

impl StdMod for StdModBasic{
    type INT = i32;
    type FLOAT = f32;
    type STRING = String;
    type FUNC = FuncObj;
    type BOOL = bool;
    type EMPTY = ();

}

impl IntType for i32 {
    type BOOL = bool;

    fn create(number: i32) -> Self {
        number
    }

    fn plus(&self, other: &Self) -> Self {
        self + other
    }

    fn minus(&self, other: &Self) -> Self {
        self - other
    }

    fn lt(&self, other: &Self) -> Self::BOOL {
        self < other
    }
}

impl FloatType for f32 {
    type BOOL = bool;

    fn create(number: f32) -> Self where Self: Sized {
        number
    }

    fn plus(&self, other: &Self) -> Self {
        self+other
    }

    fn minus(&self, other: &Self) -> Self {
        self-other
    }

    fn lt(&self, other: &Self) -> Self::BOOL {
        self<other
    }
}

impl StringType for String {
    fn create(txt: String) -> Self {
        txt
    }
}
#[derive(Clone)]
pub struct FuncObj{
    expr: Expr,
    params: Vec<String>,
    closure: HashMap<String, RefDataObj<StdModBasic>>
}

impl FuncType for FuncObj {
    type S = StdModBasic;

    fn create(expr: Expr, params: Vec<String>, closure: HashMap<String, RefDataObj<Self::S>>) -> Self {
        Self{expr, params, closure}
    }

    fn call(&self, params: Vec<RefDataObj<Self::S>>) -> RefDataObj<Self::S> {
        let mut variables = VariableStack::new();
        variables.add_variable("self_fn".to_string(),Self::S::func_obj((*self).clone()).into_ref());
        for (name, obj) in self.params.iter().zip(params){
            variables.add_variable(name.clone(),obj);
        };
        for (name, obj) in &self.closure{
            variables.add_variable(name.clone(),obj.clone())
        };

        self.expr.run(&variables)
    }
}

impl BoolType for bool {
    fn create(value: bool) -> Self {
        value
    }

    fn is_true(&self) -> bool {
        *self
    }
}

impl EmptyType for () {
    fn create() -> Self {
        ()
    }
}

