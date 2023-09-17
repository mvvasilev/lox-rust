use std::fmt::Display;

use crate::expr::{BinaryOperator, Expression, UnaryOperator};

pub struct PrettyPrinter {}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter {}
    }

    pub fn pretty_print(&self, expr: Box<Expression>) -> String {
        let mut buffer = String::new();

        match *expr {
            Expression::Binary {
                left,
                operator,
                right,
            } => buffer.push_str(&format!(
                "({} {} {})",
                self.pretty_print(left),
                operator,
                self.pretty_print(right)
            )),
            Expression::Unary { operator, right } => {
                buffer.push_str(&format!("({} {})", operator, self.pretty_print(right)))
            }
            Expression::Comma { expressions } => {
                let e = expressions
                    .iter()
                    .cloned()
                    .map(|e| self.pretty_print(e))
                    .collect::<Vec<String>>()
                    .join(",");

                buffer.push_str(&format!("({})", e));
            }
            Expression::Grouping { expression } => {
                buffer.push_str(&format!("({})", self.pretty_print(expression)));
            }
            Expression::LiteralNumber(n) => buffer.push_str(&format!("{}", n)),
            Expression::LiteralBoolean(b) => buffer.push_str(&format!("{}", b)),
            Expression::LiteralString(s) => buffer.push_str(&s),
            Expression::Nil => buffer.push_str("nil"),
            Expression::Variable(s) => buffer.push_str(&format!("var {}", s)),
        }

        buffer
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::LiteralNumber(n) => write!(f, "{}", n),
            Expression::LiteralBoolean(b) => write!(f, "{}", b),
            Expression::LiteralString(s) => write!(f, "{}", s),
            Expression::Nil => write!(f, "nil"),
            Expression::Variable(s) => write!(f, "var {}", s),
            e => write!(f, "{:?}", e),
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Minus => write!(f, "-"),
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::Multiplication => write!(f, "*"),
            BinaryOperator::Division => write!(f, "/"),
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
        }
    }
}
