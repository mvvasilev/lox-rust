use std::mem;

use crate::{
    err::LoxError,
    expr::{BinaryOperator, Expression, UnaryOperator},
    scan::Scanner,
    token::{Token, TokenKind},
};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self { scanner }
    }

    fn match_next(&mut self, args: &[TokenKind]) -> Option<Token> {
        for ele in args {
            match self.scanner.peek() {
                Some(Ok(t)) => {
                    if mem::discriminant(&t.kind) == mem::discriminant(ele) {
                        self.scanner.pop();

                        return Some(t);
                    }
                }
                Some(Err(_e)) => continue,
                None => continue,
            }
        }

        return None;
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

    pub fn parse(&mut self) -> Option<Box<Expression>> {
        match self.expression() {
            Ok(b) => Some(b),
            Err(e) => {
                print!("{}", e);

                None
            }
        }
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
            Some(Token {
                kind: TokenKind::Number(n),
                ..
            }) => Ok(Box::new(Expression::LiteralNumber(n))),
            Some(Token {
                kind: TokenKind::String(s),
                ..
            }) => Ok(Box::new(Expression::LiteralString(s))),
            Some(Token {
                kind: TokenKind::Boolean(b),
                ..
            }) => Ok(Box::new(Expression::LiteralBoolean(b))),
            Some(_) => Err(LoxError::with_line("Unsupported expression", 0)),
            None => Err(LoxError::with_line("Expected expression.", 0)),
        }
    }
}
