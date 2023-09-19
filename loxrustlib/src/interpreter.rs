use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    callable::Callable,
    environment::{Environment, Identifier},
    err::LoxError,
    expr::{BinaryOperator, Expression, LogicalOperator, UnaryOperator},
    stmt::Statement,
    token::Token,
};

pub struct Interpreter {
    environment: Box<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new(None);

        struct ClockFunc {}

        impl Callable for ClockFunc {
            fn new() -> Self
            where
                Self: Sized,
            {
                Self {}
            }

            fn arity(&self) -> usize {
                0
            }

            fn call(
                &self,
                _: &mut Interpreter,
                _: &Vec<Expression>,
            ) -> Result<Expression, LoxError> {
                Ok(Expression::LiteralNumber(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|e| LoxError::with_message(&e.to_string()))?
                        .as_secs_f64(),
                ))
            }
        }

        globals.define_callable(
            Identifier {
                name: "clock".to_string(),
            },
            Box::new(ClockFunc::new()),
        );

        let boxed_globals = Box::new(globals);

        Self {
            environment: Box::new(Environment::new(Some(boxed_globals))),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), LoxError> {
        for s in statements {
            if let Err(e) = self.execute(&s) {
                self.dump_environment();

                return Err(e);
            }
        }

        Ok(())
    }

    fn dump_environment(&mut self) {
        self.environment.print_vars(0);
    }

    fn execute(&mut self, statement: &Statement) -> Result<(), LoxError> {
        match statement {
            Statement::ExpressionStatement { expression } => {
                self.evaluate(expression)?;

                Ok(())
            }
            Statement::PrintStatement { printable } => {
                self.print(printable)?;

                Ok(())
            }
            Statement::VariableDeclaration {
                identifier,
                initializer,
            } => {
                self.declare_variable(identifier.clone(), initializer)?;

                Ok(())
            }
            Statement::BlockStatement { statements } => {
                self.execute_block_statement(statements)?;

                Ok(())
            }
            Statement::IfStatement {
                condition,
                true_branch,
                else_branch,
            } => {
                self.execute_if(condition, &*true_branch, else_branch)?;

                Ok(())
            }
            Statement::WhileStatement { condition, body } => {
                self.execute_while(condition, &*body)?;

                Ok(())
            }
        }
    }

    fn print(&mut self, expr: &Expression) -> Result<(), LoxError> {
        let result = self.evaluate(expr)?;

        println!("{}", result);

        Ok(())
    }

    fn execute_if(
        &mut self,
        condition: &Expression,
        true_branch: &Statement,
        else_branch: &Option<Box<Statement>>,
    ) -> Result<(), LoxError> {
        let condition_result = self.evaluate(condition)?;

        if condition_result != Expression::LiteralBoolean(true) {
            let Some(el) = else_branch else { return Ok(()); };

            self.execute(el)?;
        } else {
            self.execute(true_branch)?;
        }

        Ok(())
    }

    fn execute_while(&mut self, condition: &Expression, body: &Statement) -> Result<(), LoxError> {
        loop {
            let result = self.evaluate(condition)?;

            if !self.is_truthy(&result) {
                break;
            }

            self.execute(body)?;
        }

        Ok(())
    }

    fn evaluate(&mut self, expression: &Expression) -> Result<Expression, LoxError> {
        match expression {
            Expression::Assignment {
                identifier,
                expression,
            } => self.eval_assignment_expression(identifier.clone(), &*expression),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.eval_binary_expression(&*left, *operator, &*right),
            Expression::Unary { operator, right } => self.eval_unary_expression(*operator, &*right),
            Expression::Comma { expressions } => self.eval_comma_expression(&expressions),
            Expression::Grouping { expression } => self.evaluate(&*expression),
            Expression::Logical {
                left,
                operator,
                right,
            } => self.eval_logical_expression(&*left, *operator, &*right),
            Expression::Call {
                callee,
                closing_parenthesis,
                arguments,
            } => self.eval_call_expression(&*callee, closing_parenthesis, &arguments), // Avoid clone/copy?
            Expression::Identifier(t) => {
                let Some(v) = self.environment.get(&t.into()) else { return Err(LoxError::with_message(&format!("Use of undefined variable '{}'", t))); };

                Ok(v.clone())
            }
            Expression::LiteralNumber(n) => Ok(Expression::LiteralNumber(*n)), // Avoid clone/copy?
            Expression::LiteralBoolean(b) => Ok(Expression::LiteralBoolean(*b)), // Avoid clone/copy?
            Expression::LiteralString(s) => Ok(Expression::LiteralString(s.clone())), // Avoid clone/copy?
            Expression::Nil => Ok(Expression::Nil),
        }
    }

    fn execute_block_statement(&mut self, statements: &Vec<Statement>) -> Result<(), LoxError> {
        let parent_env = std::mem::take(&mut self.environment);

        self.environment = Box::new(Environment::new(Some(parent_env)));

        for stmt in statements {
            self.execute(stmt)?;
        }

        let Some(parent) = self.environment.parent.take() else { return Err(LoxError::with_message("No parent environment exists")); };
        self.environment = parent.into();

        Ok(())
    }

    fn declare_variable(
        &mut self,
        identifier: Token,
        initializer: &Option<Expression>,
    ) -> Result<(), LoxError> {
        let init = initializer
            .as_ref()
            .map(|init| self.evaluate(init))
            .transpose()?;

        self.environment.define(identifier.into(), init);

        Ok(())
    }

    fn eval_assignment_expression(
        &mut self,
        identifier: Token,
        expression: &Expression,
    ) -> Result<Expression, LoxError> {
        let value = self.evaluate(expression)?;

        self.environment.assign(identifier.into(), value.clone())?;

        Ok(value)
    }

    fn eval_comma_expression(
        &mut self,
        expressions: &Vec<Expression>,
    ) -> Result<Expression, LoxError> {
        let mut last_result = None;

        expressions.iter().rev().for_each(|expr| {
            last_result = Some(self.evaluate(&expr));
        });

        let Some(result) = last_result else { return Err(LoxError::with_message("Cannot have an empty comma statement")); };

        Ok(result?)
    }

    fn eval_unary_expression(
        &mut self,
        operator: UnaryOperator,
        right: &Expression,
    ) -> Result<Expression, LoxError> {
        match operator {
            UnaryOperator::Minus => {
                let Expression::LiteralNumber(right_num) = self.evaluate(right)? else { return Err(LoxError::with_message("Only a number can be negated this way")); };

                Ok(Expression::LiteralNumber(-right_num))
            }
            UnaryOperator::Not => {
                let left_result = self.evaluate(right)?;

                Ok(Expression::LiteralBoolean(!self.is_truthy(&left_result)))
            }
        }
    }

    fn eval_logical_expression(
        &mut self,
        left: &Expression,
        operator: LogicalOperator,
        right: &Expression,
    ) -> Result<Expression, LoxError> {
        let left_result = self.evaluate(left)?;

        if operator == LogicalOperator::Or && self.is_truthy(&left_result) {
            return Ok(left_result);
        }

        if operator == LogicalOperator::And && !self.is_truthy(&left_result) {
            return Ok(left_result);
        }

        self.evaluate(right)
    }

    fn eval_call_expression(
        &mut self,
        callee: &Expression,
        closing_parenthesis: &Token,
        arguments: &Vec<Expression>,
    ) -> Result<Expression, LoxError> {
        let callee = self.evaluate(callee)?;

        let Expression::Identifier(i) = callee else {
            return Err(LoxError::with_message_line(format!("Invalid identifier for function call '{}'", callee), closing_parenthesis.line));
        };

        let identifier = &i.into();

        let Some(callable) = self.environment.get_callable(identifier) else { // Immutable borrow of 'self' here
            return Err(LoxError::with_message_line(
                format!("Call to undefined function '{}'", identifier.name),
                closing_parenthesis.line,
            ));
        };

        if arguments.len() != callable.arity() {
            return Err(LoxError::with_message_line(
                format!(
                    "Function '{}' requires {} arguments, but was provided {}.",
                    identifier.name,
                    callable.arity(),
                    arguments.len()
                ),
                closing_parenthesis.line,
            ));
        }

        // All of this complains because of the immutable borrow of 'self' above, and this is trying to mutably borrow 'self'
        callable.call(self, &arguments.into_iter().map(|a| self.evaluate(a)).collect::<Result<Vec<_>, _>>()?)
    }

    fn eval_binary_expression(
        &mut self,
        left: &Expression,
        operator: BinaryOperator,
        right: &Expression,
    ) -> Result<Expression, LoxError> {
        let l = &self.evaluate(left)?;
        let r = &self.evaluate(right)?;

        match operator {
            BinaryOperator::Minus => self.numeric_operation(
                l,
                r,
                "Subtraction requires both operands to be numbers",
                |n1, n2| Ok(n1 - n2),
            ),
            BinaryOperator::Plus => {
                if let Expression::LiteralString(_) = l {
                    return Ok(Expression::LiteralString(format!("{}{}", l, r)));
                }

                if let Expression::LiteralString(_) = r {
                    return Ok(Expression::LiteralString(format!("{}{}", l, r)));
                }

                self.numeric_operation(
                    l,
                    r,
                    "Addition requires both operands to be numbers",
                    |n1, n2| Ok(n1 + n2),
                )
            }
            BinaryOperator::NotEqual => self.comparison(
                l,
                r,
                |n1, n2| n1 != n2,
                |s1, s2| s1 != s2,
                |b1, b2| b1 != b2,
            ),
            BinaryOperator::GreaterThanOrEqual => self.comparison(
                l,
                r,
                |n1, n2| n1 >= n2,
                |s1, s2| s1 >= s2,
                |b1, b2| b1 >= b2,
            ),
            BinaryOperator::LessThanOrEqual => self.comparison(
                l,
                r,
                |n1, n2| n1 <= n2,
                |s1, s2| s1 <= s2,
                |b1, b2| b1 <= b2,
            ),
            BinaryOperator::Equal => self.comparison(
                l,
                r,
                |n1, n2| n1 == n2,
                |s1, s2| s1 == s2,
                |b1, b2| b1 == b2,
            ),
            BinaryOperator::GreaterThan => {
                self.comparison(l, r, |n1, n2| n1 > n2, |s1, s2| s1 > s2, |b1, b2| b1 > b2)
            }
            BinaryOperator::LessThan => {
                self.comparison(l, r, |n1, n2| n1 < n2, |s1, s2| s1 < s2, |b1, b2| b1 < b2)
            }
            BinaryOperator::Multiplication => self.numeric_operation(
                l,
                r,
                "Multiplication requires both operands to be numbers",
                |n1, n2| Ok(n1 * n2),
            ),
            BinaryOperator::Division => self.numeric_operation(
                l,
                r,
                "Division requires both operands to be numbers",
                |n1, n2| {
                    if n2 == 0.0 {
                        return Err(LoxError::with_message("Division by 0"));
                    }

                    Ok(n1 / n2)
                },
            ),
        }
    }

    fn numeric_operation<N>(
        &self,
        left: &Expression,
        right: &Expression,
        invalid_operands_message: &str,
        op: N,
    ) -> Result<Expression, LoxError>
    where
        N: Fn(f64, f64) -> Result<f64, LoxError>,
    {
        let Expression::LiteralNumber(left_num) = left else { return Err(LoxError::with_message(invalid_operands_message)); };
        let Expression::LiteralNumber(right_num) = right else { return Err(LoxError::with_message(invalid_operands_message)); };

        Ok(Expression::LiteralNumber(op(*left_num, *right_num)?))
    }

    fn comparison<N, S, B>(
        &self,
        left: &Expression,
        right: &Expression,
        n: N,
        s: S,
        b: B,
    ) -> Result<Expression, LoxError>
    where
        N: Fn(f64, f64) -> bool,
        S: Fn(&str, &str) -> bool,
        B: Fn(bool, bool) -> bool,
    {
        match left {
            Expression::LiteralNumber(left_num) => {
                if right == &Expression::Nil {
                    return Ok(Expression::LiteralBoolean(false));
                }

                let Expression::LiteralNumber(right_num) = right else { return Err(LoxError::with_message("Cannot compare unlike types")); };

                Ok(Expression::LiteralBoolean(n(*left_num, *right_num)))
            }
            Expression::LiteralString(left_string) => {
                if right == &Expression::Nil {
                    return Ok(Expression::LiteralBoolean(false));
                }

                let Expression::LiteralString(right_string) = right else { return Err(LoxError::with_message("Cannot compare unlike types")); };

                Ok(Expression::LiteralBoolean(s(left_string, right_string)))
            }
            Expression::LiteralBoolean(left_bool) => {
                if right == &Expression::Nil {
                    return Ok(Expression::LiteralBoolean(false));
                }

                let Expression::LiteralBoolean(right_bool) = right else { return Err(LoxError::with_message("Cannot compare unlike types")); };

                Ok(Expression::LiteralBoolean(b(*left_bool, *right_bool)))
            }
            Expression::Nil => {
                if right == &Expression::Nil {
                    Ok(Expression::LiteralBoolean(true))
                } else {
                    Ok(Expression::LiteralBoolean(false))
                }
            }
            _ => Err(LoxError::with_message("Invalid expression for comparison")),
        }
    }

    fn is_truthy(&self, expr: &Expression) -> bool {
        if expr == &Expression::Nil {
            return false;
        }

        if expr == &Expression::LiteralBoolean(true) {
            return true;
        }

        if expr == &Expression::LiteralBoolean(false) {
            return false;
        }

        true
    }
}
