use std::{iter::Peekable, mem};

use crate::{
    err::LoxError,
    expr::{BinaryOperator, Expression, LogicalOperator, UnaryOperator},
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

    fn match_next_token(&mut self, args: &[TokenKind]) -> Option<Token> {
        let Some(Ok(next_token)) = self.scanner.peek().cloned() else { return None; };

        if Parser::match_peeked_token(&next_token.kind, args) {
            self.scanner.next(); // only consume the token if it matched

            return Some(next_token.clone()); // Yes, the token is one of the ones in the arguments - return it
        }

        None // No, the token isn't any of the ones provided - return none
    }

    fn match_next_kind(&mut self, args: &[TokenKind]) -> Option<TokenKind> {
        self.match_next_token(args).map(|t| t.kind)
    }

    fn consume_next(
        &mut self,
        expected_kind: &TokenKind,
    ) -> Result<Token, LoxError> {
        let Some(Ok(Token { kind, line, .. })) = self.scanner.peek() else { return Err(LoxError::with_message("Unexpected end of scan")) };

        if kind == expected_kind {
            let Some(Ok(token)) = self.scanner.next() else { return Err(LoxError::with_message("Token is of unexpected kind")); };

            // if-let with && is not supported. Bummer.
            return Ok(token);
        }

        Err(LoxError::with_message_line(format!("Expected token of kind '{}', instead got {}", expected_kind, kind), *line))
    }

    fn check_next(&mut self, expected_kind: &TokenKind) -> bool {
        let Some(Ok(Token { kind, .. })) = self.scanner.peek() else { return false; };
        return expected_kind == kind;
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

    fn parse_token_as_logical_op(&self, token: &Token) -> Result<LogicalOperator, LoxError> {
        match token.kind {
            TokenKind::And => Ok(LogicalOperator::And),
            TokenKind::Or => Ok(LogicalOperator::Or),
            _ => Err(LoxError::with_message_line(
                format!("Expected logical operator, got {}", token.kind),
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
        if let Some(TokenKind::Var) = self.match_next_kind(&[TokenKind::Var])
        {
            return self.variable_declaration_statement();
        }

        self.statement()
    }

    fn variable_declaration_statement(&mut self) -> Result<Statement, LoxError> {
        let Some(identifier) = self.match_next_token(&[TokenKind::Identifier(String::default())]) else { 
            return Err(LoxError::with_message("Expected variable identifier.")); 
        };

        let mut initializer = None;

        if self.match_next_token(&[TokenKind::Equal]).is_some() {
            initializer = Some(self.expression()?);
        }

        self.consume_next(&TokenKind::Semicolon)?;

        Ok(Statement::VariableDeclaration {
            identifier,
            initializer,
        })
    }

    fn statement(&mut self) -> Result<Statement, LoxError> {
        if let Some(TokenKind::Print) = self.match_next_kind(&[TokenKind::Print])
        {
            return self.print_statement();
        }

        if let Some(TokenKind::LeftBrace) = self.match_next_kind(&[TokenKind::LeftBrace])
        {
            return self.block_statement();
        }

        if let Some(TokenKind::If) = self.match_next_kind(&[TokenKind::If])
        {
            return self.if_statement();
        }

        if let Some(TokenKind::While) = self.match_next_kind(&[TokenKind::While])
        {
            return self.while_statement();
        }

        if let Some(TokenKind::For) = self.match_next_kind(&[TokenKind::For]) {
            return self.for_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Statement, LoxError> {
        let value = self.expression()?;

        self.consume_next(&TokenKind::Semicolon)?;

        Ok(Statement::PrintStatement { printable: value })
    }

    fn if_statement(&mut self) -> Result<Statement, LoxError> {
        self.consume_next(&TokenKind::LeftParen)?;

        let condition = self.expression()?;

        self.consume_next(&TokenKind::RightParen)?;

        let true_statement = self.statement()?;

        let mut else_statement = None;
        if let Some(TokenKind::Else) = self.match_next_kind(&[TokenKind::Else])
        {
            else_statement = Some(Box::new(self.statement()?));
        }

        Ok(Statement::IfStatement {
            condition,
            true_branch: Box::new(true_statement),
            else_branch: else_statement,
        })
    }

    fn while_statement(&mut self) -> Result<Statement, LoxError> {
        self.consume_next(&TokenKind::LeftParen)?;

        let condition = self.expression()?;

        self.consume_next(&TokenKind::RightParen)?;

        let body = self.statement()?;

        Ok(Statement::WhileStatement {
            condition,
            body: Box::new(body),
        })
    }

    fn for_statement(&mut self) -> Result<Statement, LoxError> {
        self.consume_next(&TokenKind::LeftParen)?;

        let initializer;

        if self.check_next(&TokenKind::Semicolon) {
            self.consume_next(&TokenKind::Semicolon)?;
            initializer = None;
        } else {
            if self.check_next(&TokenKind::Var) {
                self.consume_next(&TokenKind::Var)?;
                initializer = Some(self.variable_declaration_statement()?);
            } else {
                initializer = Some(self.expression_statement()?);
            }
        }

        let mut condition = None;

        if !self.check_next(&TokenKind::Semicolon) {
            condition = Some(self.expression()?);
        }

        self.consume_next(&TokenKind::Semicolon)?;

        let mut increment = None;

        if !self.check_next(&TokenKind::RightParen) {
            increment = Some(self.expression()?);
        }

        self.consume_next(&TokenKind::RightParen)?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = Statement::BlockStatement { statements: vec![
                body,
                Statement::ExpressionStatement { expression: incr }
            ]}
        }

        body = Statement::WhileStatement { 
            condition: condition.map_or(Expression::LiteralBoolean(true), |c| c), 
            body: Box::new(body) 
        };

        if let Some(init) = initializer {
            body = Statement::BlockStatement { statements: vec![
                init,
                body
            ]}
        }

        Ok(body)
    }

    fn block_statement(&mut self) -> Result<Statement, LoxError> {
        let mut statements = Vec::new();

        loop {
            let Some(Ok(Token { kind, .. })) = self.scanner.peek() else { break; };

            if kind == &TokenKind::RightBrace {
                break;
            }

            statements.push(self.declaration()?);
        }

        self.consume_next(&TokenKind::RightBrace)?;

        Ok(Statement::BlockStatement { statements })
    }

    fn expression_statement(&mut self) -> Result<Statement, LoxError> {
        let value = self.expression()?;

        self.consume_next(&TokenKind::Semicolon)?;

        Ok(Statement::ExpressionStatement { expression: value })
    }

    fn expression(&mut self) -> Result<Expression, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, LoxError> {
        let expr = self.or()?;

        let Some(Ok(previous)) = self.scanner.peek().cloned() else { return Err(LoxError::with_message("Invalid assignment expression")); };

        if let Some(_) = self.match_next_token(&[TokenKind::Equal]) {
            let value = self.assignment()?;

            if let Expression::Variable(v) = expr {
                return Ok(Expression::Assignment {
                    identifier: v,
                    expression: Box::new(value),
                });
            }

            return Err(LoxError::with_line(
                "Invalid assignment target",
                previous.line,
            ));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.and()?;

        loop {
            let Some(op_token) = self.match_next_token(&[TokenKind::Or]) else { break; };
            let operator = self.parse_token_as_logical_op(&op_token)?;

            expr = self.comparison().map(|right| Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })?;
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.equality()?;

        loop {
            let Some(op_token) = self.match_next_token(&[TokenKind::And]) else { break; };
            let operator = self.parse_token_as_logical_op(&op_token)?;

            expr = self.comparison().map(|right| Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })?;
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.comparison()?;

        loop {
            let Some(op_token) = self.match_next_token(&[TokenKind::BangEqual, TokenKind::EqualEqual]) else { break; };
            let operator = self.parse_token_as_binary_op(&op_token)?;

            expr = self.comparison().map(|right| Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })?;
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.term()?;

        loop {
            let Some(op_token) = self.match_next_token(&[TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual]) else { break; };
            let operator = self.parse_token_as_binary_op(&op_token)?;

            expr = self.term().map(|right| Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })?;
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.factor()?;

        loop {
            let Some(op_token) = self.match_next_token(&[TokenKind::Minus, TokenKind::Plus]) else { break; };
            let operator = self.parse_token_as_binary_op(&op_token)?;

            expr = self.factor().map(|right| Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })?;
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, LoxError> {
        let mut expr = self.unary()?;

        loop {
            let Some(op_token) = self.match_next_token(&[TokenKind::Slash, TokenKind::Star]) else { break; };
            let operator = self.parse_token_as_binary_op(&op_token)?;

            expr = self.unary().map(|right| Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })?;
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, LoxError> {
        let Some(op_token) = self.match_next_token(&[TokenKind::Bang, TokenKind::Minus]) else { return self.primary(); };
        let operator = self.parse_token_as_unary_op(&op_token)?;

        let right = self.unary()?;

        return Ok(Expression::Unary {
            operator,
            right: Box::new(right),
        });
    }

    fn primary(&mut self) -> Result<Expression, LoxError> {
        match self.match_next_token(&[
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
                ..
            }) => {
                let expr = self.expression()?;
                let Some(Ok(next_token)) = self.scanner.next() else { return Err(LoxError::with_line("Expected closing parenthesis ')'.", line)) };

                match next_token.kind {
                    TokenKind::RightParen => Ok(Expression::Grouping {
                        expression: Box::new(expr),
                    }),
                    TokenKind::Comma => {
                        let mut expressions: Vec<Expression> = Vec::new();

                        expressions.push(expr);

                        loop {
                            let next_expr = self.expression()?;

                            expressions.push(next_expr);

                            let Some(Ok(next_token)) = self.scanner.next() else { return Err(LoxError::with_line("Expected comma ',' or closing parenthesis ')'.", line)) };

                            return match next_token.kind {
                                TokenKind::RightParen => Ok(Expression::Comma { expressions }),
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
            }) => Ok(Expression::LiteralNumber(n)),
            Some(Token {
                kind: TokenKind::String(s),
                ..
            }) => Ok(Expression::LiteralString(s)),
            Some(Token {
                kind: TokenKind::Boolean(b),
                ..
            }) => Ok(Expression::LiteralBoolean(b)),
            Some(Token {
                kind: TokenKind::Nil,
                ..
            }) => Ok(Expression::Nil),
            Some(t) if matches!(t.kind, TokenKind::Identifier(_)) => Ok(Expression::Variable(t)),
            Some(t) => Err(LoxError::with_line("Unexpected token '{}'.", t.line)),
            None => Err(LoxError::with_line(
                &format!("Expected expression. Got {:?}", self.scanner.peek()),
                0,
            )),
        }
    }
}
