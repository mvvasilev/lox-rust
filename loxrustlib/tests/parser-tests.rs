use loxrustlib::{expr::BinaryOperator, expr::Expression, parser::Parser, scan::Scanner};

#[test]
pub fn parser_equality_test() {
    let expected_tree = Box::new(Expression::Binary {
        left: Box::new(Expression::LiteralNumber(5.0)),
        operator: BinaryOperator::Equal,
        right: Box::new(Expression::LiteralNumber(5.0)),
    });

    let input = "5 == 5".to_string();

    let scanner = Scanner::new(&input);
    let mut parser = Parser::new(scanner);

    let output = parser.parse();

    assert!(output.is_ok());
    assert_eq!(
        format!("{:?}", expected_tree),
        format!("{:?}", output.unwrap())
    );
}
