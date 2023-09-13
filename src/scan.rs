use std::{collections::VecDeque, str::Chars};

use crate::{
    err::LoxError,
    token::{Token, TokenKind},
};
pub struct Scanner {
    reader: Chars,
    next_chars: VecDeque<char>,
    line: usize,
    next_token: Option<Token>,
}

impl Scanner {
    pub fn new(string: String) -> Self {
        Self {
            reader: string.chars(),
            next_chars: VecDeque::new(),
            line: 1,
            next_token: None,
        }
    }

    fn pop_char(&mut self) -> Option<char> {
        match self.next_chars.pop_front() {
            Some(c) => Some(c),
            None => self.reader.next(),
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        match self.next_chars.pop_front() {
            Some(c) => {
                self.next_chars.push_front(c);

                return Some(c);
            }
            None => match self.reader.next() {
                Some(c) => {
                    self.next_chars.push_back(c);

                    return Some(c);
                }
                None => return None,
            },
        }
    }

    fn next_char_matches(&mut self, expected_char: char) -> bool {
        match self.peek_char() {
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
            match self.pop_char() {
                Some('"') => break,
                Some(c) => {
                    if c == '\n' {
                        self.line += 1;
                    }

                    buf.push(c)
                }
                None => {
                    return Err(LoxError::with_line(
                        "Unterminated string".to_string(),
                        self.line,
                    ))
                }
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
            match self.peek_char() {
                Some(c) if self.is_digit(c) || c == '.' => {
                    buf.push(c);
                    self.pop_char();
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
            match self.peek_char() {
                Some(c) if self.is_alphanumeric(c) => {
                    buf.push(c);
                    self.pop_char();
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
            "and" => Some(self.create_token(TokenKind::AND)),
            "class" => Some(self.create_token(TokenKind::CLASS)),
            "else" => Some(self.create_token(TokenKind::ELSE)),
            "false" => Some(self.create_token(TokenKind::Boolean(false))),
            "for" => Some(self.create_token(TokenKind::FOR)),
            "fun" => Some(self.create_token(TokenKind::FUN)),
            "if" => Some(self.create_token(TokenKind::IF)),
            "nil" => Some(self.create_token(TokenKind::NIL)),
            "or" => Some(self.create_token(TokenKind::OR)),
            "print" => Some(self.create_token(TokenKind::PRINT)),
            "super" => Some(self.create_token(TokenKind::SUPER)),
            "this" => Some(self.create_token(TokenKind::THIS)),
            "true" => Some(self.create_token(TokenKind::Boolean(true))),
            "var" => Some(self.create_token(TokenKind::VAR)),
            "while" => Some(self.create_token(TokenKind::WHILE)),
            "return" => Some(self.create_token(TokenKind::RETURN)),
            _ => None,
        }
    }

    fn match_next_token(&mut self) -> Option<Result<Token, LoxError>> {
        loop {
            let c = self.pop_char();

            let token = match c {
                Some('(') => self.create_token(TokenKind::LeftParen),
                Some(')') => self.create_token(TokenKind::RightParen),
                Some('{') => self.create_token(TokenKind::LeftBrace),
                Some('}') => self.create_token(TokenKind::RightBrace),
                Some(',') => self.create_token(TokenKind::COMMA),
                Some('.') => self.create_token(TokenKind::DOT),
                Some('-') => self.create_token(TokenKind::MINUS),
                Some('+') => self.create_token(TokenKind::PLUS),
                Some(';') => self.create_token(TokenKind::SEMICOLON),
                Some('*') => self.create_token(TokenKind::STAR),
                Some('!') => match self.next_char_matches('=') {
                    true => self.create_token(TokenKind::BangEqual),
                    false => self.create_token(TokenKind::BANG),
                },
                Some('=') => match self.next_char_matches('=') {
                    true => self.create_token(TokenKind::EqualEqual),
                    false => self.create_token(TokenKind::EQUAL),
                },
                Some('<') => match self.next_char_matches('=') {
                    true => self.create_token(TokenKind::LessEqual),
                    false => self.create_token(TokenKind::LESS),
                },
                Some('>') => match self.next_char_matches('=') {
                    true => self.create_token(TokenKind::GreaterEqual),
                    false => self.create_token(TokenKind::GREATER),
                },
                Some('/') => match self.next_char_matches('/') {
                    // if there's a second slash, this is a comment - pop characters until the next newline
                    true => match self.reader.find(|&next_c| next_c == '\n') {
                        Some(_) => continue,
                        None => self.create_token(TokenKind::EOF),
                    },
                    false => self.create_token(TokenKind::SLASH),
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
                    return Some(Err(LoxError::with_line(
                        format!("Unknown character '{}'", other_char),
                        self.line,
                    )))
                }
                None => self.create_token(TokenKind::EOF),
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

impl Iterator for Scanner {
    type Item = Result<Token, LoxError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token.clone() {
            Some(t) => {
                self.next_token = None;

                return Some(Ok(t));
            }
            None => self.match_next_token(),
        }
    }
}
