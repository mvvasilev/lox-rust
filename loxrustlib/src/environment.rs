use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

use crate::funcs::callable::Callable;
use crate::{err::LoxError, expr::Expression, token::Token};
use crate::outcome::Outcome;
use crate::outcome::BreakReason::Errored;
use crate::outcome::BreakReason::Returned;
use std::collections::hash_map::Entry::Occupied;

#[derive(Default)]
pub struct Environment {
    pub parent: Option<Rc<RefCell<Environment>>>,

    callables: HashMap<Identifier, Rc<dyn Callable>>,
    variables: HashMap<Identifier, Option<Expression>>,
}

impl Environment {
    pub fn new(parent: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            parent,
            callables: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn get_callable(&self, name: &Identifier) -> Option<Rc<dyn Callable>> {
        match self.callables.get(name) {
            Some(callable) => Some(callable.clone()),
            None => self.parent.as_ref().map(|e| e.borrow().get_callable(name))?,
        }
    }

    pub fn define_callable(&mut self, name: Identifier, callable: Rc<dyn Callable>) {
        self.callables.insert(name, callable);
    }

    pub fn assign(&mut self, name: &Identifier, value: Expression) -> Outcome<()> {
        if let Occupied(mut e) = self.variables.entry(name.clone()) {
            e.insert(Some(value));

            Ok(())
        } else {
            self.parent.as_mut().map_or(
                Err(Errored(LoxError::with_message(&format!(
                    "Could not assign nonexistent identifier '{}'",
                    name.name
                )))),
                |e| e.borrow_mut().assign(name, value),
            )
        }
    }

    pub fn define(&mut self, name: Identifier, value: Option<Expression>) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &Identifier) -> Option<Expression> {
        match self.variables.get(name) {
            Some(v) => v.clone(),
            None => {
                match &self.parent {
                    Some(p) => {
                        p.borrow().get(name)
                    },
                    None => None,
                }
            },
        }
    }

    pub fn print_vars(&self, level: usize) {
        for (k, v) in &self.variables {
            println!("{}. {}: {:?}", level, k.name, v);
        }

        if let Some(upper) = &self.parent {
            upper.borrow().print_vars(level + 1);
        }
    }

    pub fn get_at(&self, distance: usize, identifier: &Identifier) -> Option<Expression> {
        if distance == 0 {
            return self.get(identifier);
        }

        self.parent.as_ref().and_then(|p| p.borrow().get_at(distance - 1, identifier))
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
