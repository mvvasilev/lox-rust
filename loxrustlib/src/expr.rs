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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
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
        expressions: Vec<Box<Expression>>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    LiteralNumber(f64),
    LiteralBoolean(bool),
    LiteralString(String),
    Nil,
    Variable(String),
}
