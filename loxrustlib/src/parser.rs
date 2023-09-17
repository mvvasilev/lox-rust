use std::{iter::Peekable, mem};

use crate::{
    err::LoxError,
    expr::{BinaryOperator, Expression, UnaryOperator},
    scan::Scanner,
    stmt::Statement,
    token::{Token, TokenKind},
};

pub struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner: scanner.peekable(),
        }
    }

    fn match_peeked_token(peeked: &TokenKind, args: &[TokenKind]) -> bool {
        for ele in args {
            if mem::discriminant(peeked) == mem::discriminant(ele) {
                return true;
            }
        }

        return false;
    }

    fn match_next(&mut self, args: &[TokenKind]) -> Option<Token> {
        let Some(Ok(next_token)) = self.scanner.peek().cloned() else { return None; };

        if Parser::match_peeked_token(&next_token.kind, args) {
            self.scanner.next(); // only consume the token if it matched

            // println!("Marched {}", &next_token.kind);

            return Some(next_token.clone()); // Yes, the token is one of the ones in the arguments - return it
        }

        None // No, the token isn't any of the ones provided - return none
    }

    fn consume_next(
        &mut self,
        expected_kind: &TokenKind,
        err_if_not_kind: LoxError,
    ) -> Result<Token, LoxError> {
        if let Some(Ok(Token { kind, .. })) = self.scanner.peek() {
            if kind == expected_kind {
                let Some(Ok(token)) = self.scanner.next() else { return Err(LoxError::with_message("Token is of unexpected kind")); };

                // if-let with && is not supported. Bummer.
                return Ok(token);
            }
        }

        Err(err_if_not_kind)
    }

    fn parse_token_as_unary_op(&self, token: &Token) -> Result<UnaryOperator, LoxError> {
        match token.kind {
            TokenKind::Bang => Ok(UnaryOperator::Not),
            TokenKind::Minus => Ok(UnaryOperator::Minus),
            _ => Err(LoxError::with_message_line(
                format!("Expected unary operator, got {}", token.kind),
                token.line,
            )),
        }
    }

    fn parse_token_as_binary_op(&self, token: &Token) -> Result<BinaryOperator, LoxError> {
        match token.kind {
            TokenKind::Plus => Ok(BinaryOperator::Plus),
            TokenKind::Minus => Ok(BinaryOperator::Minus),
            TokenKind::BangEqual => Ok(BinaryOperator::NotEqual),
            TokenKind::EqualEqual => Ok(BinaryOperator::Equal),
            TokenKind::GreaterEqual => Ok(BinaryOperator::GreaterThanOrEqual),
            TokenKind::Greater => Ok(BinaryOperator::GreaterThan),
            TokenKind::LessEqual => Ok(BinaryOperator::LessThanOrEqual),
            TokenKind::Less => Ok(BinaryOperator::LessThan),
            TokenKind::Slash => Ok(BinaryOperator::Division),
            TokenKind::Star => Ok(BinaryOperator::Multiplication),
            _ => Err(LoxError::with_message_line(
                format!("Expected binary operator, got {}", token.kind),
                token.line,
            )),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, LoxError> {
        let mut result = Vec::new();

        loop {
            if let Some(Ok(Token {
                kind: TokenKind::Eof,
                ..
            })) = self.scanner.peek()
            {
                break;
            }

            result.push(self.declaration()?);
        }

        Ok(result)
    }

    fn declaration(&mut self) -> Result<Statement, LoxError> {
        if let Some(Token {
            kind: TokenKind::Var,
            ..
        }) = self.match_next(&[TokenKind::Var])
        {
            return self.variable_declaration_statement();
        }

        self.statement()
    }

    fn variable_declaration_statement(&mut self) -> Result<Statement, LoxError> {
        let Some(Token { kind: TokenKind::Identifier(identifier), .. }) = self.match_next(&[TokenKind::Identifier(String::default())]) else { 
            return Err(LoxError::with_message("Expected variable identifier.")); 
        };

        let mut initializer = None;

        if self.match_next(&[TokenKind::Equal]).is_some() {
            initializer = Some(self.expression()?);
        }

        self.consume_next(&TokenKind::Semicolon, LoxError::with_message("Expected semicolon ';' to terminate statement."))?;

        Ok(Statement::VariableDeclaration {
            identifier,
            initializer,
        })
    }

    fn statement(&mut self) -> Result<Statement, LoxError> {
        if let Some(Token {
            kind: TokenKind::Print,
            ..
        }) = self.match_next(&[TokenKind::Print])
        {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Statement, LoxError> {
        let value = self.expression()?;

        self.consume_next(
            &TokenKind::Semicolon,
            LoxError::with_message("Expected semicolon ';' to terminate statement."),
        )?;

        Ok(Statement::PrintStatement { printable: value })
    }

    fn expression_statement(&mut self) -> Result<Statement, LoxError> {
        let value = self.expression()?;

        self.consume_next(
            &TokenKind::Semicolon,
            LoxError::with_message("Expected semicolon ';' to terminate statement."),
        )?;

        Ok(Statement::ExpressionStatement { expression: value })
    }

    fn expression(&mut self) -> Result<Box<Expression>, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expression>, LoxError> {
        let mut expr = self.comparison()?;

        loop {
            let Some(op_token) = self.match_next(&[TokenKind::BangEqual, TokenKind::EqualEqual]) else { break; };
            let operator = self.parse_token_as_binary_op(&op_token)?;

            expr = self.comparison().map(|right| {
                Box::new(Expression::Binary {
                    left: expr,
                    operator,
                    right,
                })
            })?;
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expression>, LoxError> {
        let mut expr: Box<Expression> = self.term()?;

        loop {
            let Some(op_token) = self.match_next(&[TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual]) else { break; };
            let operator = self.parse_token_as_binary_op(&op_token)?;

            expr = self.term().map(|right| {
                Box::new(Expression::Binary {
                    left: expr,
                    operator,
                    right,
                })
            })?;
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expression>, LoxError> {
        let mut expr = self.factor()?;

        loop {
            let Some(op_token) = self.match_next(&[TokenKind::Minus, TokenKind::Plus]) else { break; };
            let operator = self.parse_token_as_binary_op(&op_token)?;

            expr = self.factor().map(|right| {
                Box::new(Expression::Binary {
                    left: expr,
                    operator,
                    right,
                })
            })?;
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expression>, LoxError> {
        let mut expr = self.unary()?;

        loop {
            let Some(op_token) = self.match_next(&[TokenKind::Slash, TokenKind::Star]) else { break; };
            let operator = self.parse_token_as_binary_op(&op_token)?;

            expr = self.unary().map(|right| {
                Box::new(Expression::Binary {
                    left: expr,
                    operator,
                    right,
                })
            })?;
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expression>, LoxError> {
        let Some(op_token) = self.match_next(&[TokenKind::Bang, TokenKind::Minus]) else { return self.primary(); };
        let operator = self.parse_token_as_unary_op(&op_token)?;

        let right = self.unary()?;

        return Ok(Box::new(Expression::Unary { operator, right }));
    }

    fn primary(&mut self) -> Result<Box<Expression>, LoxError> {
        match self.match_next(&[
            TokenKind::Nil,
            TokenKind::Boolean(bool::default()),
            TokenKind::Number(f64::default()),
            TokenKind::String(String::default()),
            TokenKind::Identifier(String::default()),
            TokenKind::LeftParen,
        ]) {
            Some(Token {
                kind: TokenKind::LeftParen,
                line,
            }) => {
                let expr = self.expression()?;
                let Some(Ok(next_token)) = self.scanner.next() else { return Err(LoxError::with_line("Expected closing parenthesis ')'.", line)) };

                match next_token.kind {
                    TokenKind::RightParen => {
                        Ok(Box::new(Expression::Grouping { expression: expr }))
                    }
                    TokenKind::Comma => {
                        let mut expressions: Vec<Box<Expression>> = Vec::new();

                        expressions.push(expr);

                        loop {
                            let next_expr = self.expression()?;

                            expressions.push(next_expr);

                            let Some(Ok(next_token)) = self.scanner.next() else { return Err(LoxError::with_line("Expected comma ',' or closing parenthesis ')'.", line)) };

                            return match next_token.kind {
                                TokenKind::RightParen => {
                                    Ok(Box::new(Expression::Comma { expressions }))
                                }
                                TokenKind::Comma => continue,
                                _ => Err(LoxError::with_line(
                                    "Expected comma ',' or closing parenthesis ')'.",
                                    line,
                                )),
                            };
                        }
                    }
                    _ => Err(LoxError::with_line(
                        "Expected closing parenthesis ')'.",
                        line,
                    )),
                }
            }
            Some(Token { kind: TokenKind::Number(n), .. }) => Ok(Box::new(Expression::LiteralNumber(n))),
            Some(Token { kind: TokenKind::String(s), .. }) => Ok(Box::new(Expression::LiteralString(s))),
            Some(Token { kind: TokenKind::Boolean(b), .. }) => Ok(Box::new(Expression::LiteralBoolean(b))),
            Some(Token { kind: TokenKind::Nil, .. }) => Ok(Box::new(Expression::Nil)),
            Some(Token { kind: TokenKind::Identifier(s), .. }) => Ok(Box::new(Expression::Variable(s))),
            Some(_) => Err(LoxError::with_line("Unsupported expression.", 0)),
            None => Err(LoxError::with_line("Expected expression.", 0)),
        }
    }
}
