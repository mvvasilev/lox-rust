use crate::{
    err::LoxError,
    expr::{BinaryOperator, Expression, UnaryOperator},
    printer::PrettyPrinter,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, expression: Box<Expression>) {
        let printer = PrettyPrinter::new();

        println!("{}", printer.pretty_print(expression.clone()));

        match self.evaluate(expression) {
            Ok(r) => println!("{}", printer.pretty_print(r)),
            Err(e) => println!("Failed to interpret, met error: {}", e),
        }
    }

    fn evaluate(&self, expression: Box<Expression>) -> Result<Box<Expression>, LoxError> {
        match *expression {
            Expression::Binary { left, operator, right } => self.eval_binary_expression(left, operator, right),
            Expression::Unary { operator, right } => self.eval_unary_expression(operator, right),
            Expression::Comma { expressions } => self.eval_comma_expression(expressions),
            Expression::Grouping { expression } => self.evaluate(expression),
            _ => Ok(expression),
        }
    }

    fn eval_comma_expression(&self, expressions: Vec<Box<Expression>>) -> Result<Box<Expression>, LoxError> {
        let Some(last) = expressions.last() else { return Err(LoxError::with_message("Invalid comma operator")); };

        self.evaluate(last.clone()) // bad
    }

    fn eval_unary_expression(&self, operator: UnaryOperator, right: Box<Expression>) -> Result<Box<Expression>, LoxError> {
        match operator {
            UnaryOperator::Minus => {
                let Expression::LiteralNumber(right_num) = *self.evaluate(right)? else { return Err(LoxError::with_message("Only a number can be negated this way")); };

                Ok(Box::new(Expression::LiteralNumber(-right_num)))
            },
            UnaryOperator::Not => {
                let Expression::LiteralBoolean(right_bool) = *self.evaluate(right)? else { return Err(LoxError::with_message("Only a booleans can be negated this way")); };

                Ok(Box::new(Expression::LiteralBoolean(!right_bool)))
            },
        }
    }

    fn eval_binary_expression(&self, left: Box<Expression>, operator: BinaryOperator, right: Box<Expression>) -> Result<Box<Expression>, LoxError> {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;

        match operator {
            BinaryOperator::Minus => self.numeric_operation(l, r, "Subtraction requires both operands to be numbers", |n1, n2| Ok(n1 - n2)),
            BinaryOperator::Plus => self.numeric_operation(l, r, "Addition requires both operands to be numbers", |n1, n2| Ok(n1 + n2)),
            BinaryOperator::NotEqual => self.comparison(l, r, |n1, n2| n1 != n2, |s1, s2| s1 != s2, |b1, b2| b1 != b2),
            BinaryOperator::GreaterThanOrEqual => self.comparison(l, r, |n1, n2| n1 >= n2, |s1, s2| s1 >= s2, |b1, b2| b1 >= b2),
            BinaryOperator::LessThanOrEqual => self.comparison(l, r, |n1, n2| n1 <= n2, |s1, s2| s1 <= s2, |b1, b2| b1 <= b2),
            BinaryOperator::Equal => self.comparison(l, r, |n1, n2| n1 == n2, |s1, s2| s1 == s2, |b1, b2| b1 == b2),
            BinaryOperator::GreaterThan => self.comparison(l, r, |n1, n2| n1 > n2, |s1, s2| s1 > s2, |b1, b2| b1 > b2),
            BinaryOperator::LessThan => self.comparison(l, r, |n1, n2| n1 < n2, |s1, s2| s1 < s2, |b1, b2| b1 < b2),
            BinaryOperator::Multiplication => self.numeric_operation(l, r, "Multiplication requires both operands to be numbers", |n1, n2| Ok(n1 * n2)),
            BinaryOperator::Division => self.numeric_operation(l, r, "Division requires both operands to be numbers", |n1, n2| {
                if n2 == 0.0 {
                    return Err(LoxError::with_message("Division by 0"));
                }

                Ok(n1 / n2)
            }),
        }
    }

    fn numeric_operation<N>(&self, left: Box<Expression>, right: Box<Expression>, invalid_operands_message: &str, op: N) -> Result<Box<Expression>, LoxError> where N: Fn(f64, f64) -> Result<f64, LoxError>
    {
        let Expression::LiteralNumber(left_num) = *left else { return Err(LoxError::with_message(invalid_operands_message)); };
        let Expression::LiteralNumber(right_num) = *right else { return Err(LoxError::with_message(invalid_operands_message)); };

        Ok(Box::new(Expression::LiteralNumber(op(left_num, right_num)?)))
    }

    fn comparison<N, S, B>(
        &self,
        left: Box<Expression>,
        right: Box<Expression>,
        n: N,
        s: S,
        b: B,
    ) -> Result<Box<Expression>, LoxError>
    where
        N: Fn(f64, f64) -> bool,
        S: Fn(String, String) -> bool,
        B: Fn(bool, bool) -> bool,
    {
        match *left {
            Expression::LiteralNumber(left_num) => {
                let Expression::LiteralNumber(right_num) = *right else { return Err(LoxError::with_message("Cannot compare unlike types")); };

                Ok(Box::new(Expression::LiteralBoolean(n(
                    left_num, 
                    right_num,
                ))))
            }
            Expression::LiteralString(left_string) => {
                let Expression::LiteralString(right_string) = *right else { return Err(LoxError::with_message("Cannot compare unlike types")); };

                Ok(Box::new(Expression::LiteralBoolean(s(
                    left_string,
                    right_string,
                ))))
            }
            Expression::LiteralBoolean(left_bool) => {
                let Expression::LiteralBoolean(right_bool) = *right else { return Err(LoxError::with_message("Cannot compare unlike types")); };

                Ok(Box::new(Expression::LiteralBoolean(b(
                    left_bool, 
                    right_bool,
                ))))
            }
            _ => Err(LoxError::with_message("Invalid expression for comparison")),
        }
    }
}
