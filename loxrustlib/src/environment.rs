use std::collections::HashMap;

use crate::{expr::Expression, err::LoxError, token::Token};

pub struct Environment {
    pub parent: Option<Box<Environment>>,
    values: HashMap<Identifier, Option<Expression>>
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Self { parent, values: HashMap::new() }
    }

    pub fn assign(&mut self, name: Identifier, value: Expression) -> Result<(), LoxError> {
        if self.values.contains_key(&name) {
            self.values.insert(name.clone(), Some(value));

            return Ok(());
        }

        self.parent.as_mut().map_or(
            Err(LoxError::with_message_line(format!("Could not assign nonexistent identifier '{}'", name.name), name.line)),
            |e| e.assign(name, value)
        )
    }

    pub fn define(&mut self, name: Identifier, value: Option<Expression>) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Identifier) -> Option<Expression> {
        match self.values.get(&name) {
            Some(v) => v.clone(),
            None => self.parent.as_mut().map(|e| e.get(name))?
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self { parent: Default::default(), values: Default::default() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    name: String,
    line: usize
}

impl From<Token> for Identifier {
    fn from(value: Token) -> Self {
        Self {
            name: value.lexeme,
            line: value.line
        }
    }
}