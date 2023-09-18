use crate::{expr::Expression, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExpressionStatement {
        expression: Box<Expression>,
    },
    PrintStatement {
        printable: Box<Expression>,
    },
    VariableDeclaration {
        identifier: Token,
        initializer: Option<Box<Expression>>,
    },
    BlockStatement {
        statements: Vec<Statement>
    }
}
