use crate::{err::LoxError, expr::Expression, interpreter::Interpreter};

pub trait Callable {
    fn new() -> Self
    where
        Self: Sized;

    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &Vec<Expression>,
    ) -> Result<Expression, LoxError>;
}
