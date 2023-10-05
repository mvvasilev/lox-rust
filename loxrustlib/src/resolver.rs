use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::Interpreter;
use crate::{stmt::Statement, outcome::Outcome, expr::Expression, token::Token, err::LoxError};
use crate::outcome::BreakReason::Errored;
use crate::outcome::BreakReason::Returned;

struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new()
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn resolve(&mut self, statements: &[Statement]) -> Outcome<()> {
        for s in statements {
            self.resolve_statement(s)?;
        }

        Ok(())
    }

    fn resolve_statement(&mut self, statement: &Statement) -> Outcome<()> {
        match statement {
            Statement::ExpressionStatement { expression } => {
                self.resolve_expression(expression)?;
            },
            Statement::PrintStatement { printable } => {
                self.resolve_expression(printable)?;
            },
            Statement::VariableDeclaration { identifier, initializer } => {
                self.declare_var(identifier);

                initializer.as_ref().map(|init| self.resolve_expression(init));

                self.define_var(identifier);
            },
            Statement::BlockStatement { statements } => {
                self.resolve(statements)?;
            },
            Statement::IfStatement { condition, true_branch, else_branch } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(true_branch)?;

                if let Some(el) = else_branch {
                    self.resolve_statement(el)?;
                }
            },
            Statement::WhileStatement { condition, body } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(body)?;
            },
            Statement::FunDeclaration { name, parameters, body } => {
                self.declare_var(name);
                self.define_var(name);

                self.resolve_function(parameters, body)?;
            },
            Statement::ReturnStatement { keyword: _, value } => {
                if let Expression::Nil = value {
                    return Ok(())
                }

                self.resolve_expression(value)?;
            },
        }

        Ok(())
    }

    fn resolve_expression(&mut self, expression: &Expression) -> Outcome<Expression> {
        match expression {
            Expression::Assignment { id, identifier, expression } => {
                self.resolve_expression(expression)?;

                self.resolve_local(*id, identifier);
            },
            Expression::Binary { left, operator: _, right } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            },
            Expression::Unary { operator: _, right } => {
                self.resolve_expression(right)?;
            },
            Expression::Comma { expressions } => {
                for expr in expressions {
                    self.resolve_expression(expr)?;
                }
            },
            Expression::Grouping { expression } => {
                self.resolve_expression(expression)?;
            },
            Expression::Logical { left, operator: _, right } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            },
            Expression::Call { callee, closing_parenthesis: _, arguments } => {
                self.resolve_expression(callee)?;

                for expr in arguments {
                    self.resolve_expression(expr)?;
                }
            },
            Expression::LiteralNumber(_) => {},
            Expression::LiteralBoolean(_) => {},
            Expression::LiteralString(_) => {},
            Expression::Nil => {},
            Expression::Identifier(id, t) => {
                if let Some(l) = self.scopes.last() { 
                    if l.contains_key(&t.lexeme) {
                        return Err(Errored(LoxError::with_line("Unable to read local variable in its own initializer", t.line)));
                    }
                }

                self.resolve_local(*id, t);
            },
        }

        Ok(Expression::Nil)
    }

    fn resolve_local(&mut self, expression_id: u16, token: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&token.lexeme) {
                self.interpreter.resolve(expression_id, self.scopes.len() - 1 - i);
            }
        }
    }

    fn resolve_function(&mut self, parameters: &[Token], body: &[Statement]) -> Outcome<()> {
        self.begin_scope();

        for param in parameters {
            self.declare_var(param);
            self.define_var(param);
        }

        self.resolve(body)?;

        self.end_scope();

        Ok(())
    }

    fn declare_var(&mut self, name: &Token) {
        self.scopes.last_mut().map(|l| l.insert(name.lexeme.clone(), false));
    }

    fn define_var(&mut self, name: &Token) {
        self.scopes.last_mut().map(|l| l.insert(name.lexeme.clone(), true));
    }

}