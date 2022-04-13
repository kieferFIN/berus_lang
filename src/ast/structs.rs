use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::ast::types::TypeInfo;

#[derive(Eq)]
pub struct StructDef {
    pub name: String,
    pub members: HashMap<String, TypeInfo>,
}

impl PartialEq for StructDef {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for StructDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}