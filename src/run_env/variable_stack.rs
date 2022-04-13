use std::collections::HashMap;
use crate::run_env::RefDataObj;
use crate::std_library::StdMod;

pub struct VariableStack<S: StdMod>{
    stack: Vec<HashMap<String, RefDataObj<S>>>
}

impl<S: StdMod> VariableStack<S> {

    pub fn new()->Self{
        Self{stack: vec![HashMap::new()]}
    }

    pub fn add_variable(&mut self, name:String, v: RefDataObj<S>){
        self.stack.last_mut().unwrap().insert(name, v);
    }

    pub fn find_variable(&self, name:&str) -> RefDataObj<S>{
        self.try_find_variable(name).expect(&format!("Runtime ERROR: Variable {} not found.",name))
    }

    pub fn try_find_variable(&self, name:&str) -> Option<RefDataObj<S>>{
        for layer in self.stack.iter().rev(){
            match layer.get(name) {
                Some(d) => return Some(d.clone()),
                _ => ()
            };
        };
        None
    }

    pub fn add_layer(&mut self) {
        self.stack.push(HashMap::new())
    }

    pub fn pop_layer(&mut self) {
        self.stack.pop();
        if self.stack.is_empty() {
            self.add_layer()
        }
    }

    pub fn extend(&mut self, mut other: VariableStack<S>)->Result<(),String>{
        if other.stack.len() != 1{
            Err(format!("VariableStack should have only 1 layer. Found: {}",other.stack.len()))
        }else {
            for (name,obj) in other.stack.pop().unwrap(){
                self.stack[0].insert(name, obj);
            };
            Ok(())
        }
    }

}