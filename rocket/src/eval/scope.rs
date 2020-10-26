
use std::collections::HashMap;
use crate::eval::Value;

pub struct Scope {
    vals: HashMap<String, Value>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            vals: HashMap::new(),
        }
    }

    pub fn get(&self, k: String) -> Option<&Value> {
        self.vals.get(&k)
    }

    pub fn set(&mut self, k: String, v: Value) {
        self.vals.insert(k, v);
    }
}
