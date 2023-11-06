use crate::{expr::Expression, interpreter::Interpreter, outcome::Outcome};

pub trait Callable {
    fn arity(&self) -> usize;

    fn call(&self, interpreter: &mut Interpreter, args: &[Expression]) -> Outcome<Expression>;
}
