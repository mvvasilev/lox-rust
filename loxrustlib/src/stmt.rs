use crate::{expr::Expression, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExpressionStatement {
        expression: Expression,
    },
    PrintStatement {
        printable: Expression,
    },
    VariableDeclaration {
        identifier: Token,
        initializer: Option<Expression>,
    },
    BlockStatement {
        statements: Vec<Statement>,
    },
    IfStatement {
        condition: Expression,
        true_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    WhileStatement {
        condition: Expression,
        body: Box<Statement>,
    },
    FunDeclaration {
        name: Token,
        parameters: Vec<Token>,
        body: Vec<Statement>,
    },
    ReturnStatement {
        keyword: Token,
        value: Expression,
    },
}
