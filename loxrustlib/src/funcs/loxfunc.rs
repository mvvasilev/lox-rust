use crate::{stmt::Statement, token::Token, interpreter::Interpreter, expr::Expression, outcome::Outcome, environment::Environment};

use super::callable::Callable;

pub struct LoxDefinedFunction {
    parameters: Vec<Token>,
    body: Vec<Statement>
}

impl LoxDefinedFunction {
    pub fn new(parameters: Vec<Token>, body: Vec<Statement>) -> Self
    where
        Self: Sized {
        Self {
            parameters, body
        }
    }
}

impl Callable for LoxDefinedFunction {
    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: &[Expression]) -> Outcome<Expression> {
        let mut env = Environment::new(Some(interpreter.global_env.clone()));
        
        for (i, param_name) in self.parameters.iter().enumerate() {
            env.define(param_name.clone().into(), args.get(i).cloned());
        }

        
        interpreter.execute_block_statement(&self.body, env)?;

        Ok(Expression::Nil)
    }
}