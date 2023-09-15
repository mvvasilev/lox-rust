use std::mem;

use crate::{
    err::LoxError,
    expr::{Binary, Expression, Grouping, Literal, Unary},
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

    pub fn parse(&mut self) -> Option<Box<dyn Expression>> {
        match self.expression() {
            Ok(b) => Some(b),
            Err(e) => {
                print!("{}", e);

                None
            }
        }
    }

    fn expression(&mut self) -> Result<Box<dyn Expression>, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<dyn Expression>, LoxError> {
        let mut expr = self.comparison()?;

        loop {
            let Some(operator) = self.match_next(&[TokenKind::BangEqual, TokenKind::EqualEqual]) else { break; };

            expr = self
                .comparison()
                .map(|right| Box::new(Binary::new(expr, operator, right)))?;
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<dyn Expression>, LoxError> {
        let mut expr: Box<dyn Expression> = self.term()?;

        loop {
            let Some(operator) = self.match_next(&[TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual]) else { break; };

            expr = self
                .term()
                .map(|right| Box::new(Binary::new(expr, operator, right)))?;
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<dyn Expression>, LoxError> {
        let mut expr = self.factor()?;

        loop {
            let Some(operator) = self.match_next(&[TokenKind::Minus, TokenKind::Plus]) else { break; };

            expr = self
                .factor()
                .map(|right| Box::new(Binary::new(expr, operator, right)))?;
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<dyn Expression>, LoxError> {
        let mut expr = self.unary()?;

        loop {
            let Some(operator) = self.match_next(&[TokenKind::Slash, TokenKind::Star]) else { break; };

            expr = self
                .unary()
                .map(|right| Box::new(Binary::new(expr, operator, right)))?;
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<dyn Expression>, LoxError> {
        let Some(operator) = self.match_next(&[TokenKind::Bang, TokenKind::Minus]) else { return self.primary(); };

        let right = self.unary()?;

        return Ok(Box::new(Unary::new(operator, right)));
    }

    fn primary(&mut self) -> Result<Box<dyn Expression>, LoxError> {
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

                if next_token.kind == TokenKind::RightParen {
                    Ok(Box::new(Grouping::new(expr)))
                } else {
                    Err(LoxError::with_line(
                        "Expected closing parenthesis ')'.",
                        line,
                    ))
                }
            }
            Some(t) => Ok(Box::new(Literal::new(t))),
            None => Err(LoxError::with_line("Expected expression.", 0)),
        }
    }
}
