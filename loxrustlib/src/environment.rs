use std::{collections::HashMap, rc::Rc};

use crate::{interpreter::Callable, err::LoxError, expr::Expression, token::Token};
use crate::outcome::Outcome;
use crate::outcome::BreakReason::Errored;
use crate::outcome::BreakReason::Returned;

#[derive(Default)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,

    callables: HashMap<Identifier, Rc<dyn Callable>>,
    variables: HashMap<Identifier, Option<Expression>>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Self {
            parent,
            callables: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn get_callable(&self, name: &Identifier) -> Option<Rc<dyn Callable>> {
        match self.callables.get(name) {
            Some(callable) => Some(callable.clone()),
            None => self.parent.as_ref().map(|e| e.get_callable(name))?,
        }
    }

    pub fn define_callable(&mut self, name: Identifier, callable: Rc<dyn Callable>) {
        self.callables.insert(name, callable);
    }

    pub fn assign(&mut self, name: &Identifier, value: Expression) -> Outcome<()> {
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.variables.entry(name.clone()) {
            e.insert(Some(value));

            Ok(())
        } else {
            self.parent.as_mut().map_or(
                Err(Errored(LoxError::with_message(&format!(
                    "Could not assign nonexistent identifier '{}'",
                    name.name
                )))),
                |e| e.assign(name, value),
            )
        }
    }

    pub fn define(&mut self, name: Identifier, value: Option<Expression>) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &Identifier) -> Option<&Expression> {
        match self.variables.get(name) {
            Some(v) => v.as_ref(),
            None => self.parent.as_ref().map(|e| e.get(name))?,
        }
    }

    pub fn print_vars(&self, level: usize) {
        for (k, v) in &self.variables {
            println!("{}. {}: {:?}", level, k.name, v);
        }

        let mut current = self;

        while let Some(upper) = &current.parent {
            upper.print_vars(level + 1);

            current = upper;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl From<Token> for Identifier {
    fn from(value: Token) -> Self {
        Self { name: value.lexeme }
    }
}

impl From<&Token> for Identifier {
    fn from(value: &Token) -> Self {
        Self {
            name: value.lexeme.clone(),
        }
    }
}

impl From<&&Token> for Identifier {
    fn from(value: &&Token) -> Self {
        Self {
            name: value.lexeme.clone(),
        }
    }
}
