use crate::expr::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExpressionStatement { expression: Box<Expression> },
    PrintStatement { printable: Box<Expression> },
}
