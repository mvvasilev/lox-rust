use std::time::{SystemTime, UNIX_EPOCH};

use crate::{interpreter::Interpreter, expr::Expression, outcome::Outcome, err::LoxError};
use crate::outcome::BreakReason::Errored;

use super::callable::Callable;

pub struct ClockFunc {}

impl ClockFunc {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}

impl Default for ClockFunc {
    fn default() -> Self {
        Self::new()
    }
}

impl Callable for ClockFunc {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _: &mut Interpreter,
        _: &[Expression],
    ) -> Outcome<Expression> {
        Ok(Expression::LiteralNumber(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| Errored(LoxError::with_message(&e.to_string())))?
                .as_secs_f64(),
        ))
    }
}