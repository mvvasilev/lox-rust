use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Minus,
    Plus,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    GreaterThan,
    LessThan,
    Multiplication,
    Division,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Assignment {
        identifier: Token,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    Comma {
        expressions: Vec<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: LogicalOperator,
        right: Box<Expression>,
    },
    LiteralNumber(f64),
    LiteralBoolean(bool),
    LiteralString(String),
    Nil,
    Variable(Token),
}
