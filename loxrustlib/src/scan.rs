use std::{iter::Peekable, str::Chars};

use crate::{
    err::LoxError,
    token::{Token, TokenKind},
};
pub struct Scanner<'a> {
    reader: Peekable<Chars<'a>>,
    line: usize,
    next_token: Option<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(string: &'a String) -> Self {
        Self {
            reader: string.chars().peekable(),
            line: 1,
            next_token: None,
        }
    }

    fn peek_match(&mut self, expected_char: char) -> bool {
        match self.reader.peek().cloned() {
            Some(c) => c == expected_char,
            None => false,
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alphabetical(&mut self, alpha_char: char) -> bool {
        alpha_char.is_alphabetic() || alpha_char == '_'
    }

    fn is_alphanumeric(&mut self, c: char) -> bool {
        self.is_alphabetical(c) || self.is_digit(c)
    }

    fn create_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.line)
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

        Ok(Token::new(
            TokenKind::String(String::from_iter(buf)),
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

        Ok(Token::new(
            TokenKind::Number(String::from_iter(buf).parse().unwrap()),
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

        match self.match_keyword(&ident) {
            Some(token) => return Ok(token),
            None => return Ok(self.create_token(TokenKind::Identifier(ident))),
        }
    }

    fn match_keyword(&self, identifier: &str) -> Option<Token> {
        match identifier {
            "and" => Some(self.create_token(TokenKind::And)),
            "class" => Some(self.create_token(TokenKind::Class)),
            "else" => Some(self.create_token(TokenKind::Else)),
            "false" => Some(self.create_token(TokenKind::Boolean(false))),
            "for" => Some(self.create_token(TokenKind::For)),
            "fun" => Some(self.create_token(TokenKind::Fun)),
            "if" => Some(self.create_token(TokenKind::If)),
            "nil" => Some(self.create_token(TokenKind::Nil)),
            "or" => Some(self.create_token(TokenKind::Or)),
            "print" => Some(self.create_token(TokenKind::Print)),
            "super" => Some(self.create_token(TokenKind::Super)),
            "this" => Some(self.create_token(TokenKind::This)),
            "true" => Some(self.create_token(TokenKind::Boolean(true))),
            "var" => Some(self.create_token(TokenKind::Var)),
            "while" => Some(self.create_token(TokenKind::While)),
            "return" => Some(self.create_token(TokenKind::Return)),
            _ => None,
        }
    }

    fn match_next_token(&mut self) -> Option<Result<Token, LoxError>> {
        loop {
            let c = self.reader.next();

            let token = match c {
                Some('(') => self.create_token(TokenKind::LeftParen),
                Some(')') => self.create_token(TokenKind::RightParen),
                Some('{') => self.create_token(TokenKind::LeftBrace),
                Some('}') => self.create_token(TokenKind::RightBrace),
                Some(',') => self.create_token(TokenKind::Comma),
                Some('.') => self.create_token(TokenKind::Dot),
                Some('-') => self.create_token(TokenKind::Minus),
                Some('+') => self.create_token(TokenKind::Plus),
                Some(';') => self.create_token(TokenKind::Semicolon),
                Some('*') => self.create_token(TokenKind::Star),
                Some('!') => match self.peek_match('=') {
                    true => {
                        self.reader.next();

                        self.create_token(TokenKind::BangEqual)
                    }
                    false => self.create_token(TokenKind::Bang),
                },
                Some('=') => match self.peek_match('=') {
                    true => {
                        self.reader.next();

                        self.create_token(TokenKind::EqualEqual)
                    }
                    false => self.create_token(TokenKind::Equal),
                },
                Some('<') => match self.peek_match('=') {
                    true => {
                        self.reader.next();

                        self.create_token(TokenKind::LessEqual)
                    }
                    false => self.create_token(TokenKind::Less),
                },
                Some('>') => match self.peek_match('=') {
                    true => {
                        self.reader.next();

                        self.create_token(TokenKind::GreaterEqual)
                    }
                    false => self.create_token(TokenKind::Greater),
                },
                Some('/') => match self.peek_match('/') {
                    // if there's a second slash, this is a comment - pop characters until the next newline
                    true => match self.reader.find(|&next_c| next_c == '\n') {
                        Some(_) => continue,
                        None => self.create_token(TokenKind::Eof),
                    },
                    false => self.create_token(TokenKind::Slash),
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
                None => self.create_token(TokenKind::Eof),
            };

            return Some(Ok(token));
        }
    }

    pub fn peek(&mut self) -> Option<Result<Token, LoxError>> {
        self.next_token.clone().map(|t| Ok(t)).or_else(|| {
            Some(self.next()?.map(|t| {
                self.next_token = Some(t.clone());

                t
            }))
        })
    }

    pub fn pop(&mut self) -> Option<Result<Token, LoxError>> {
        self.next()
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, LoxError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token.take() {
            Some(t) => {
                self.next_token = None;

                return Some(Ok(t));
            }
            None => self.match_next_token(),
        }
    }
}
