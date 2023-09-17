use loxrustlib::{
    scan::Scanner,
    token::{Token, TokenKind},
};

#[test]
pub fn test_scanner_every_token() {
    let input = "( ) { } , . - + ; / * ! != = == > >= < <= asdf \"asdf\" 76.12 true and class else fun for if nil or print return super this var while".to_string();
    let expected_output = &[
        token_of(TokenKind::LeftParen),
        token_of(TokenKind::RightParen),
        token_of(TokenKind::LeftBrace),
        token_of(TokenKind::RightBrace),
        token_of(TokenKind::Comma),
        token_of(TokenKind::Dot),
        token_of(TokenKind::Minus),
        token_of(TokenKind::Plus),
        token_of(TokenKind::Semicolon),
        token_of(TokenKind::Slash),
        token_of(TokenKind::Star),
        token_of(TokenKind::Bang),
        token_of(TokenKind::BangEqual),
        token_of(TokenKind::Equal),
        token_of(TokenKind::EqualEqual),
        token_of(TokenKind::Greater),
        token_of(TokenKind::GreaterEqual),
        token_of(TokenKind::Less),
        token_of(TokenKind::LessEqual),
        token_of(TokenKind::Identifier("asdf".to_string())),
        token_of(TokenKind::String("asdf".to_string())),
        token_of(TokenKind::Number(76.12)),
        token_of(TokenKind::Boolean(true)),
        token_of(TokenKind::And),
        token_of(TokenKind::Class),
        token_of(TokenKind::Else),
        token_of(TokenKind::Fun),
        token_of(TokenKind::For),
        token_of(TokenKind::If),
        token_of(TokenKind::Nil),
        token_of(TokenKind::Or),
        token_of(TokenKind::Print),
        token_of(TokenKind::Return),
        token_of(TokenKind::Super),
        token_of(TokenKind::This),
        token_of(TokenKind::Var),
        token_of(TokenKind::While),
    ];

    assert_token_stream_equality(expected_output, input);
}

#[test]
pub fn test_scanner_matches_double_equals() {
    let input = "==".to_string();
    let expected_output = &[token_of(TokenKind::EqualEqual)];

    assert_token_stream_equality(expected_output, input);
}

#[test]
pub fn test_scanner_matches_comparison_equal_equal() {
    let input = "5 == 6".to_string();
    let expected_output = &[
        token_of(TokenKind::Number(5.0)),
        token_of(TokenKind::EqualEqual),
        token_of(TokenKind::Number(6.0)),
    ];

    assert_token_stream_equality(expected_output, input);
}

#[test]
pub fn test_scanner_matches_comparison_not_equal() {
    let input = "5 != 6".to_string();
    let expected_output = &[
        token_of(TokenKind::Number(5.0)),
        token_of(TokenKind::BangEqual),
        token_of(TokenKind::Number(6.0)),
    ];

    assert_token_stream_equality(expected_output, input);
}

fn assert_token_stream_equality(expected: &[Token], input: String) {
    let mut scanner = Scanner::new(&input);

    let mut i = 0;
    loop {
        if i >= expected.len() {
            break;
        }

        let expected_token = &expected[i];

        let output_token = scanner.next().unwrap().unwrap();

        assert_eq!(expected_token, &output_token);

        i += 1;
    }
}

fn token_of(kind: TokenKind) -> Token {
    token_of_at(kind, 1)
}

fn token_of_at(kind: TokenKind, lexeme: &str, line: usize) -> Token {
    Token { kind, lexeme: lexeme.to_string(), line }
}
