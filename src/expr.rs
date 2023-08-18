trait Expression {}

struct BinaryExpression {
    left: &dyn Expression,
    operator: Token,
    right: &dyn Expression
}

impl Expression for BinaryExpression {}