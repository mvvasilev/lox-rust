use std::fmt::Display;

use crate::{err::LoxError, expr::Expression};

pub type Outcome<T> = Result<T, BreakReason>;

#[derive(Clone)]
pub enum BreakReason {
    Errored(LoxError),
    Returned(Expression),
}

impl Display for BreakReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BreakReason::Errored(e) => write!(f, "{}", e),
            BreakReason::Returned(r) => write!(f, "{}", r),
        }
    }
}
