use std::collections::HashMap;

use crate::ast::expr::Expr;
use crate::run_env::{DataObj, RefDataObj};

pub trait StdMod: Sized + Clone {
    type INT: IntType<BOOL=Self::BOOL>;
    type FLOAT: FloatType<BOOL=Self::BOOL>;
    type STRING: StringType;
    type FUNC: FuncType<S=Self>;
    type BOOL: BoolType;
    type EMPTY: EmptyType;

    fn int_create(v: i32) -> DataObj<Self> {
        DataObj::Int(Self::INT::create(v))
    }
    fn int_obj(v: Self::INT) -> DataObj<Self> {
        DataObj::Int(v)
    }

    fn float_create(v: f32) -> DataObj<Self> {
        DataObj::Float(Self::FLOAT::create(v))
    }

    fn float_obj(v: Self::FLOAT) -> DataObj<Self> {
        DataObj::Float(v)
    }

    fn string_create(txt: String) -> DataObj<Self> {
        DataObj::String(Self::STRING::create(txt))
    }

    fn func_create(expr: Expr, params: Vec<String>, closure: HashMap<String, RefDataObj<Self>>) -> DataObj<Self> {
        DataObj::Func(Self::FUNC::create(expr, params, closure))
    }
    fn func_obj(f: Self::FUNC) -> DataObj<Self> {
        DataObj::Func(f)
    }

    fn bool_create(b: bool) -> DataObj<Self> {
        DataObj::Bool(Self::BOOL::create(b))
    }

    fn bool_obj(b: Self::BOOL) -> DataObj<Self> {
        DataObj::Bool(b)
    }

    fn empty_create() -> DataObj<Self> {
        DataObj::Empty(Self::EMPTY::create())
    }
}

pub trait IntType: Clone {
    type BOOL: BoolType;

    fn create(number: i32) -> Self;
    fn plus(&self, other: &Self) -> Self;
    fn minus(&self, other: &Self) -> Self;
    fn lt(&self, other: &Self) -> Self::BOOL;
}

pub trait FloatType: Clone {
    type BOOL: BoolType;

    fn create(number: f32) -> Self;
    fn plus(&self, other: &Self) -> Self;
    fn minus(&self, other: &Self) -> Self;
    fn lt(&self, other: &Self) -> Self::BOOL;
}

pub trait StringType: Clone {
    fn create(txt: String) -> Self;
}

pub trait FuncType: Clone {
    type S: StdMod;
    fn create(expr: Expr, params: Vec<String>, closure: HashMap<String, RefDataObj<Self::S>>) -> Self;
    fn call(&self, params: Vec<RefDataObj<Self::S>>) -> RefDataObj<Self::S>;
}

pub trait BoolType: Clone {
    fn create(value: bool) -> Self;
    fn is_true(&self) -> bool;
}

pub trait EmptyType: Clone {
    fn create() -> Self;
}