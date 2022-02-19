use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::ast::types::VariableType;

pub(crate) struct VariableManager {
    variables: Vec<HashMap<String, VariableType>>,
}

impl VariableManager {
    pub fn new() -> Self {
        VariableManager {
            variables: vec![HashMap::new()]
        }
    }

    pub fn add_variable(&mut self, name: String, variable_type: VariableType) {
        self.variables.last_mut().unwrap().insert(name, variable_type);
    }
    pub fn find_variable(&self, name: &String) -> Option<VariableType> {
        for layer in self.variables.iter().rev() {
            match layer.get(name) {
                Some(vt) => return Some((*vt).clone()),
                _ => ()
            };
        };
        None
    }

    pub fn add_layer(&mut self) {
        self.variables.push(HashMap::new())
    }

    pub fn pop_layer(&mut self) {
        self.variables.pop();
        if self.variables.is_empty() {
            self.add_layer()
        }
    }
}

impl Debug for VariableManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for l in &self.variables {
            for (name, vt) in l {
                write!(f, "{}:{} ", name, vt)?
            }
        }
        Ok(())
    }
}