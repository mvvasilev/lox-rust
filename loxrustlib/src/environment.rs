use std::collections::HashMap;

use crate::{expr::Expression, err::LoxError, token::Token};

pub struct Environment {
    values: HashMap<Identifier, Option<Box<Expression>>>
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn assign(&mut self, name: Identifier, value: Box<Expression>) -> Result<(), LoxError> {
        let Some(_) = self.values.insert(name.clone(), Some(value)) else {
            return Err(LoxError::with_message_line(format!("Could not assign nonexistent identifier '{}'", name.name), name.line));
        };

        Ok(())
    }

    pub fn define(&mut self, name: Identifier, value: Option<Box<Expression>>) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Identifier) -> Option<Box<Expression>> {
        return self.values.get(&name).cloned()?;
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