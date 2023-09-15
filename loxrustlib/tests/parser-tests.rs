use loxrustlib::{
    expr::{Binary, Literal},
    parser::Parser,
    scan::Scanner,
    token::{Token, TokenKind},
};

#[test]
pub fn parser_equality_test() {
    let expected_tree = Box::new(Binary::new(
        Box::new(Literal::new(Token::new(TokenKind::Number(5.0), 1))),
        Token::new(TokenKind::EqualEqual, 1),
        Box::new(Literal::new(Token::new(TokenKind::Number(5.0), 1))),
    ));
    let input = "5 == 5".to_string();

    let scanner = Scanner::new(&input);
    let mut parser = Parser::new(scanner);

    let output = parser.parse();

    assert!(output.is_some());
    assert_eq!(
        format!("{:?}", expected_tree),
        format!("{:?}", output.unwrap())
    );
}
