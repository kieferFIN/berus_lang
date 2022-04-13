use std::cell::RefCell;
use std::rc::Rc;
use crate::std_library::StdMod;

pub type RefDataObj<S> = Rc<RefCell<DataObj<S>>>;

#[derive(Clone)]
pub enum DataObj<S: StdMod> {
    Int(S::INT),
    Float(S::FLOAT),
    String(S::STRING),
    Func(S::FUNC),
    Bool(S::BOOL),
    Empty(S::EMPTY),
}

impl<S: StdMod> DataObj<S> {
    pub fn into_ref(self) -> RefDataObj<S> {
        Rc::new(RefCell::new(self))
    }

    pub fn type_str(&self) -> &str {
        match self {
            DataObj::Int(_) => "INT",
            DataObj::Float(_) => "FLOAT",
            DataObj::String(_) => "STRING",
            DataObj::Func(_) => "FUNC",
            DataObj::Bool(_) => "BOOL",
            DataObj::Empty(_) => "EMPTY"
        }
    }
}