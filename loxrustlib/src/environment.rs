use std::collections::HashMap;

use crate::expr::Expression;

pub struct Environment {
    values: HashMap<String, Option<Box<Expression>>>
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: Option<Box<Expression>>) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &str) -> Option<Box<Expression>> {
        return self.values.get(name).cloned()?;
    }
}