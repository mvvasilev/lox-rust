use std::{iter::Peekable, str::Chars};

use crate::{
    err::LoxError,
    token::{Token, TokenKind},
};
pub struct Scanner<'a> {
    reader: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            reader: string.chars().peekable(),
            line: 1,
        }
    }

    fn peek_match(&mut self, expected_char: char) -> bool {
        match self.reader.peek().cloned() {
            Some(c) => c == expected_char,
            None => false,
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alphabetical(&mut self, alpha_char: char) -> bool {
        alpha_char.is_alphabetic() || alpha_char == '_'
    }

    fn is_alphanumeric(&mut self, c: char) -> bool {
        self.is_alphabetical(c) || self.is_digit(c)
    }

    fn create_token(&self, kind: TokenKind, lexeme: String) -> Token {
        Token::new(kind, lexeme, self.line)
    }

    fn create_token_str(&self, kind: TokenKind, lexeme: &str) -> Token {
        Token::new(kind, lexeme.to_string(), self.line)
    }

    fn create_string_token(&mut self) -> Result<Token, LoxError> {
        let mut buf: Vec<char> = Vec::new();

        loop {
            match self.reader.next() {
                Some('"') => break,
                Some('\\') => {
                    if self.reader.peek() == Some(&'"') {
                        buf.push('"');
                        self.reader.next();
                    }
                }
                Some(c) => {
                    if c == '\n' {
                        self.line += 1;
                    }

                    buf.push(c)
                }
                None => return Err(LoxError::with_line("Unterminated string", self.line)),
            }
        }

        let lexeme = String::from_iter(buf);

        Ok(Token::new(
            TokenKind::String(lexeme.clone()),
            lexeme,
            self.line,
        ))
    }

    fn create_number_token(&mut self, starting_digit: char) -> Result<Token, LoxError> {
        let mut buf: Vec<char> = Vec::new();
        buf.push(starting_digit);

        loop {
            match self.reader.peek().cloned() {
                Some(c) if self.is_digit(c) || c == '.' => {
                    buf.push(c);
                    self.reader.next();
                }
                Some(_) | None => break,
            }
        }

        let lexeme = String::from_iter(buf);

        Ok(Token::new(
            TokenKind::Number(lexeme.parse().unwrap()),
            lexeme,
            self.line,
        ))
    }

    fn create_identifier_token(&mut self, alpha_char: char) -> Result<Token, LoxError> {
        let mut buf: Vec<char> = Vec::new();
        buf.push(alpha_char);

        loop {
            match self.reader.peek().cloned() {
                Some(c) if self.is_alphanumeric(c) => {
                    buf.push(c);
                    self.reader.next();
                }
                Some(_) | None => break,
            }
        }

        let ident = String::from_iter(buf);

        match self.match_keyword(ident.clone()) {
            Some(token) => Ok(token),
            None => Ok(self.create_token(TokenKind::Identifier(ident.clone()), ident)),
        }
    }

    fn match_keyword(&self, identifier: String) -> Option<Token> {
        match identifier.as_str() {
            "and" => Some(self.create_token(TokenKind::And, identifier)),
            "class" => Some(self.create_token(TokenKind::Class, identifier)),
            "else" => Some(self.create_token(TokenKind::Else, identifier)),
            "false" => Some(self.create_token(TokenKind::Boolean(false), identifier)),
            "for" => Some(self.create_token(TokenKind::For, identifier)),
            "fun" => Some(self.create_token(TokenKind::Fun, identifier)),
            "if" => Some(self.create_token(TokenKind::If, identifier)),
            "nil" => Some(self.create_token(TokenKind::Nil, identifier)),
            "or" => Some(self.create_token(TokenKind::Or, identifier)),
            "print" => Some(self.create_token(TokenKind::Print, identifier)),
            "super" => Some(self.create_token(TokenKind::Super, identifier)),
            "this" => Some(self.create_token(TokenKind::This, identifier)),
            "true" => Some(self.create_token(TokenKind::Boolean(true), identifier)),
            "var" => Some(self.create_token(TokenKind::Var, identifier)),
            "while" => Some(self.create_token(TokenKind::While, identifier)),
            "return" => Some(self.create_token(TokenKind::Return, identifier)),
            _ => None,
        }
    }

    fn match_next_token(&mut self) -> Option<Result<Token, LoxError>> {
        loop {
            let c = self.reader.next();

            let token = match c {
                Some('(') => self.create_token_str(TokenKind::LeftParen, "("),
                Some(')') => self.create_token_str(TokenKind::RightParen, ")"),
                Some('{') => self.create_token_str(TokenKind::LeftBrace, "{"),
                Some('}') => self.create_token_str(TokenKind::RightBrace, "}"),
                Some(',') => self.create_token_str(TokenKind::Comma, ","),
                Some('.') => self.create_token_str(TokenKind::Dot, "."),
                Some('-') => self.create_token_str(TokenKind::Minus, "-"),
                Some('+') => self.create_token_str(TokenKind::Plus, "+"),
                Some(';') => self.create_token_str(TokenKind::Semicolon, ";"),
                Some('*') => self.create_token_str(TokenKind::Star, "*"),
                Some('!') => match self.peek_match('=') {
                    true => {
                        self.reader.next();

                        self.create_token_str(TokenKind::BangEqual, "!=")
                    }
                    false => self.create_token_str(TokenKind::Bang, "!"),
                },
                Some('=') => match self.peek_match('=') {
                    true => {
                        self.reader.next();

                        self.create_token_str(TokenKind::EqualEqual, "==")
                    }
                    false => self.create_token_str(TokenKind::Equal, "="),
                },
                Some('<') => match self.peek_match('=') {
                    true => {
                        self.reader.next();

                        self.create_token_str(TokenKind::LessEqual, "<=")
                    }
                    false => self.create_token_str(TokenKind::Less, "<"),
                },
                Some('>') => match self.peek_match('=') {
                    true => {
                        self.reader.next();

                        self.create_token_str(TokenKind::GreaterEqual, ">=")
                    }
                    false => self.create_token_str(TokenKind::Greater, ">"),
                },
                Some('/') => match self.peek_match('/') {
                    // if there's a second slash, this is a comment - pop characters until the next newline
                    true => match self.reader.find(|&next_c| next_c == '\n') {
                        Some(_) => continue,
                        None => self.create_token_str(TokenKind::Eof, "eof"),
                    },
                    false => self.create_token_str(TokenKind::Slash, "/"),
                },
                Some(' ') | Some('\r') | Some('\t') => continue,
                Some('\n') => {
                    self.line += 1;
                    continue;
                }
                Some('"') => return Some(self.create_string_token()),
                Some(digit_char) if self.is_digit(digit_char) => {
                    return Some(self.create_number_token(digit_char))
                }
                Some(alpha_char) if self.is_alphabetical(alpha_char) => {
                    return Some(self.create_identifier_token(alpha_char))
                }
                Some(other_char) => {
                    return Some(Err(LoxError::with_message_line(
                        format!("Unknown character '{}'", other_char),
                        self.line,
                    )))
                }
                None => self.create_token_str(TokenKind::Eof, "eof"),
            };

            return Some(Ok(token));
        }
    }

    pub fn pop(&mut self) -> Option<Result<Token, LoxError>> {
        self.next()
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, LoxError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.match_next_token()
    }
}
