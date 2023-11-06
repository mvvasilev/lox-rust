use std::{collections::HashMap, rc::Rc};

use crate::funcs::callable::Callable;
use crate::outcome::BreakReason::Errored;
use crate::outcome::BreakReason::Returned;
use crate::outcome::Outcome;
use crate::{err::LoxError, expr::Expression, token::Token};
use std::collections::hash_map::Entry::Occupied;

#[derive(Default)]
pub struct Environment {
    callables: HashMap<String, Rc<dyn Callable>>,
    variables: HashMap<String, Option<Expression>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            callables: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn get_callable(&self, name: &str) -> Option<Rc<dyn Callable>> {
        Some(self.callables.get(name)?.clone())
    }

    pub fn define_callable(&mut self, name: String, callable: Rc<dyn Callable>) {
        self.callables.insert(name, callable);
    }

    pub fn has_declared_variable(&self, ident: &str) -> bool {
        self.variables.contains_key(ident)
    }

    pub fn assign(&mut self, name: &str, value: &Expression) -> Outcome<()> {
        if let Occupied(mut e) = self.variables.entry(name.to_string()) {
            e.insert(Some(value.clone()));

            Ok(())
        } else {
            Err(Errored(LoxError::with_message(&format!(
                "Could not assign nonexistent identifier '{}'",
                name
            ))))
        }
    }

    pub fn define(&mut self, name: String, value: Option<Expression>) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Expression> {
        self.variables.get(name)?.clone()
    }

    pub fn print_vars(&self, level: usize) {
        for (k, v) in &self.variables {
            println!("{}. {}: {:?}", level, k, v);
        }
    }
}
