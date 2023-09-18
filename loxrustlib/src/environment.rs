use std::collections::HashMap;

use crate::{err::LoxError, expr::Expression, token::Token};

pub struct Environment {
    pub parent: Option<Box<Environment>>,
    values: HashMap<Identifier, Option<Expression>>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Self {
            parent,
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: Identifier, value: Expression) -> Result<(), LoxError> {
        if self.values.contains_key(&name) {
            self.values.insert(name.clone(), Some(value));

            return Ok(());
        }

        self.parent.as_mut().map_or(
            Err(LoxError::with_message(
                &format!("Could not assign nonexistent identifier '{}'", name.name)
            )),
            |e| e.assign(name, value),
        )
    }

    pub fn define(&mut self, name: Identifier, value: Option<Expression>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Identifier) -> Option<&Expression> {
        match self.values.get(name) {
            Some(v) => v.as_ref(),
            None => self.parent.as_ref().map(|e| e.get(name))?,
        }
    }

    pub fn print_vars(&self, level: usize) {
        for (k, v) in &self.values {
            println!("{}. {}: {:?}", level, k.name, v);
        }

        let mut current = self;

        while let Some(upper) = &current.parent {
            upper.print_vars(level + 1);

            current = upper;
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            parent: Default::default(),
            values: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String
}

impl From<Token> for Identifier {
    fn from(value: Token) -> Self {
        Self {
            name: value.lexeme
        }
    }
}

impl From<&Token> for Identifier {
    fn from(value: &Token) -> Self {
        Self {
            name: value.lexeme.clone()
        }
    }
}
