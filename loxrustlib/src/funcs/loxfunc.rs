use crate::{
    environment::Environment, expr::Expression, interpreter::Interpreter, outcome::Outcome,
    stmt::Statement, token::Token,
};
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

use super::callable::Callable;

pub struct LoxDefinedFunction {
    parameters: Vec<Token>,
    body: Vec<Statement>,
    parent_env: LinkedList<Rc<RefCell<Environment>>>,
}

impl LoxDefinedFunction {
    pub fn new(
        parameters: Vec<Token>,
        body: Vec<Statement>,
        parent_env: LinkedList<Rc<RefCell<Environment>>>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            parameters,
            body,
            parent_env,
        }
    }
}

impl Callable for LoxDefinedFunction {
    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: &[Expression]) -> Outcome<Expression> {
        let mut env = Environment::new();

        for (i, param_name) in self.parameters.iter().enumerate() {
            env.define(param_name.lexeme.clone(), args.get(i).cloned());
        }

        interpreter.execute_block_statement_in_environment(&self.body, self.parent_env.clone())?;

        Ok(Expression::Nil)
    }
}
