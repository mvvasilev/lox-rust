use std::{rc::Rc, collections::HashMap};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::LinkedList;
use crate::{outcome::Outcome, funcs::{clockfunc::ClockFunc, loxfunc::LoxDefinedFunction}};
use crate::outcome::BreakReason::Errored;
use crate::outcome::BreakReason::Returned;

use crate::{
    environment::Environment,
    err::LoxError,
    expr::{BinaryOperator, Expression, LogicalOperator, UnaryOperator},
    stmt::Statement,
    token::Token,
};
use crate::funcs::callable::Callable;
use crate::outcome::BreakReason;

#[derive(Default)]
pub struct Interpreter {
    environments: LinkedList<Rc<RefCell<Environment>>>,
    locals: HashMap<u16, usize>
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Rc::new(RefCell::new(Environment::new()));

        globals.borrow_mut().define_callable(
            "clock".to_string(),
            Rc::new(ClockFunc::new()),
        );


        Self {
            environments: LinkedList::from([globals]),
            locals: HashMap::new()
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Outcome<()> {
        for s in statements {
            if let Err(Errored(e)) = self.execute(&s) {
                self.dump_environment();
                println!("{:?}", &s);

                return Err(Errored(e));
            }
        }

        Ok(())
    }

    fn dump_environment(&mut self) {
        for (i, env) in self.environments.iter().enumerate() {
            env.borrow().print_vars(i);
        }
    }

    pub fn execute(&mut self, statement: &Statement) -> Outcome<()> {
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
                self.execute_if(condition, true_branch, else_branch)?;

                Ok(())
            }
            Statement::WhileStatement { condition, body } => {
                self.execute_while(condition, body)?;

                Ok(())
            }
            Statement::FunDeclaration { name, parameters, body } => {
                self.define_function(name, parameters, body)?;

                Ok(())
            },
            Statement::ReturnStatement { keyword, value } => {
                Err(Returned(self.evaluate(value)?))
            },
        }
    }

    fn print(&mut self, expr: &Expression) -> Outcome<()> {
        let result = self.evaluate(expr)?;

        println!("{}", result);

        Ok(())
    }

    fn execute_if(
        &mut self,
        condition: &Expression,
        true_branch: &Statement,
        else_branch: &Option<Box<Statement>>,
    ) -> Outcome<()> {
        let condition_result = self.evaluate(condition)?;

        if condition_result != Expression::LiteralBoolean(true) {
            let Some(el) = else_branch else { return Ok(()); };

            self.execute(el)?;
        } else {
            self.execute(true_branch)?;
        }

        Ok(())
    }

    fn execute_while(&mut self, condition: &Expression, body: &Statement) -> Outcome<()> {
        loop {
            let result = self.evaluate(condition)?;

            if !self.is_truthy(&result) {
                break;
            }

            self.execute(body)?;
        }

        Ok(())
    }

    fn define_function(&mut self, name: &Token, parameters: &[Token], body: &[Statement]) -> Outcome<()> {
        let identifier = name.lexeme.clone();

        if self.get_callable(&identifier).is_some() {
            return Err(Errored(LoxError::with_message_line(format!("Function named '{}' already exists", identifier), name.line)))
        }

        self.current_environment().borrow_mut().define_callable(identifier, Rc::new(LoxDefinedFunction::new(parameters.to_owned(), body.to_owned(), self.environments.clone())));

        Ok(())
    }

    fn evaluate(&mut self, expression: &Expression) -> Outcome<Expression> {
        match expression {
            Expression::Assignment {
                id,
                identifier,
                expression,
            } => {
                let depth = self.locals.get(id).unwrap_or(&0);

                if depth == &0 {
                    return self.eval_assignment_expression(identifier, expression)
                }

                self.eval_assignment_expression_at(identifier, depth.clone(), expression)
            },
            Expression::Binary {
                left,
                operator,
                right,
            } => self.eval_binary_expression(left, *operator, right),
            Expression::Unary { operator, right } => self.eval_unary_expression(*operator, right),
            Expression::Comma { expressions } => self.eval_comma_expression(expressions),
            Expression::Grouping { expression } => self.evaluate(expression),
            Expression::Logical {
                left,
                operator,
                right,
            } => self.eval_logical_expression(left, *operator, right),
            Expression::Call {
                callee,
                closing_parenthesis,
                arguments,
            } => self.eval_call_expression(callee, closing_parenthesis, arguments),
            Expression::Identifier(id, t) => {
                let depth = self.locals.get(id).unwrap_or(&0);

                if depth == &0 {
                    return self.eval_identifier(t)
                }

                self.eval_identifier_at(t, depth.clone())
            },
            Expression::LiteralNumber(n) => Ok(Expression::LiteralNumber(*n)),
            Expression::LiteralBoolean(b) => Ok(Expression::LiteralBoolean(*b)),
            Expression::LiteralString(s) => Ok(Expression::LiteralString(s.clone())),
            Expression::Nil => Ok(Expression::Nil)
        }
    }

    pub fn execute_block_statement(&mut self, statements: &[Statement]) -> Outcome<()> {

        self.environments.push_front(Rc::new(RefCell::new(Environment::new())));

        for stmt in statements {
            self.execute(stmt)?;
        }

        self.environments.pop_front();

        Ok(())
    }

    pub fn execute_block_statement_in_environment(&mut self, statements: &[Statement], environment: LinkedList<Rc<RefCell<Environment>>>) -> Outcome<()>  {
        let old_env = std::mem::take(&mut self.environments);

        self.environments = environment;

        for stmt in statements {
            self.execute(stmt)?;
        }

        self.environments = old_env;

        Ok(())
    }

    fn declare_variable(
        &mut self,
        identifier: Token,
        initializer: &Option<Expression>,
    ) -> Outcome<()> {
        let init = initializer
            .as_ref()
            .map(|init| self.evaluate(init))
            .transpose()?;

        self.current_environment().borrow_mut().define(identifier.lexeme.clone(), init);

        Ok(())
    }

    fn eval_assignment_expression(
        &mut self,
        identifier: &Token,
        expression: &Expression,
    ) -> Outcome<Expression> {
        let value = self.evaluate(expression)?;

        self.assign_variable(&identifier.lexeme, &value)?;

        Ok(value)
    }

    fn eval_assignment_expression_at(
        &mut self,
        identifier: &Token,
        depth: usize,
        expression: &Expression
    ) -> Outcome<Expression> {
        let value = self.evaluate(expression)?;

        self.assign_variable_at(&identifier.lexeme, &value, depth)?;

        Ok(value)
    }

    fn eval_comma_expression(
        &mut self,
        expressions: &[Expression],
    ) -> Outcome<Expression> {
        let mut last_result = None;

        expressions.iter().rev().for_each(|expr| {
            last_result = Some(self.evaluate(expr));
        });

        let Some(result) = last_result else { return Err(Errored(LoxError::with_message("Cannot have an empty comma statement"))); };

        result
    }

    fn eval_unary_expression(
        &mut self,
        operator: UnaryOperator,
        right: &Expression,
    ) -> Outcome<Expression> {
        match operator {
            UnaryOperator::Minus => {
                let Expression::LiteralNumber(right_num) = self.evaluate(right)? else { return Err(Errored(LoxError::with_message("Only a number can be negated this way"))); };

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
    ) -> Outcome<Expression> {
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
    ) -> Outcome<Expression> {
        let identifier: String;
        let depth: &usize;

        if let Expression::Identifier(id, t) = callee {
            identifier = t.lexeme.clone();
            depth = self.locals.get(id).unwrap_or(&0);
        } else {
            let Expression::Identifier(id, t) = self.evaluate(callee)? else {
                return Err(Errored(LoxError::with_message_line(format!("Invalid identifier for function call '{}'", callee), closing_parenthesis.line)));
            };

            identifier = t.lexeme.clone();
            depth = self.locals.get(&id).unwrap_or(&0);
        }

        let callable;

        if depth == &0 {
            callable = self.get_callable(&identifier);
        } else {
            callable = self.get_callable_at(&identifier, depth.clone());
        }

        let Some(c) = callable else {
            return Err(Errored(LoxError::with_message_line(
                format!("Call to undefined function '{}'", identifier),
                closing_parenthesis.line,
            )));
        };

        if arguments.len() != c.arity() {
            return Err(Errored(LoxError::with_message_line(
                format!(
                    "Function '{}' requires {} arguments, but was provided {}.",
                    identifier,
                    c.arity(),
                    arguments.len()
                ),
                closing_parenthesis.line,
            )));
        }

        let x = arguments.iter().map(|a| self.evaluate(a)).collect::<Result<Vec<_>, _>>()?;

        match c.call(self, &x) {
            Ok(e) => Ok(e),
            Err(Returned(r)) => Ok(r),
            Err(Errored(e)) => Err(Errored(e))
        }
    }

    fn eval_binary_expression(
        &mut self,
        left: &Expression,
        operator: BinaryOperator,
        right: &Expression,
    ) -> Outcome<Expression> {
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
                self.comparison(l, r, |n1, n2| n1 > n2, |s1, s2| s1 > s2, |b1, b2| b1 & !b2)
            }
            BinaryOperator::LessThan => {
                self.comparison(l, r, |n1, n2| n1 < n2, |s1, s2| s1 < s2, |b1, b2| !b1 & b2)
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
                        return Err(Errored(LoxError::with_message("Division by 0")));
                    }

                    Ok(n1 / n2)
                },
            ),
        }
    }

    fn eval_identifier(&self, token: &Token) -> Outcome<Expression> {
        if self.get_callable(&token.lexeme).is_some() {
            return Ok(Expression::LiteralString(format!("<fn {}>", token.lexeme)));
        }

        let Some(v) = self.get_variable(&token.lexeme) else {
            return Err(Errored(LoxError::with_message(&format!("Use of undefined variable '{}'", token)))); 
        };

        Ok(v)
    }

    fn eval_identifier_at(&self, token: &Token, depth: usize) -> Outcome<Expression>  {
        if self.get_callable_at(&token.lexeme, depth).is_some() {
            return Ok(Expression::LiteralString(format!("<fn {}>", token.lexeme)));
        }

        let Some(v) = self.get_variable_at(&token.lexeme, depth) else {
            return Err(Errored(LoxError::with_message(&format!("Use of undefined variable '{}'", token))));
        };

        Ok(v)
    }

    fn look_up_variable(&self, name: &Token, expression: u16) -> Option<Expression> {
        if let Some(distance) = self.locals.get(&expression) {
            self.get_variable_at(&name.lexeme, *distance)
        } else {
            self.get_variable(&name.lexeme)
        }
    }

    fn numeric_operation<N>(
        &self,
        left: &Expression,
        right: &Expression,
        invalid_operands_message: &str,
        op: N,
    ) -> Outcome<Expression>
    where
        N: Fn(f64, f64) -> Outcome<f64>,
    {
        let Expression::LiteralNumber(left_num) = left else { return Err(Errored(LoxError::with_message(invalid_operands_message))); };
        let Expression::LiteralNumber(right_num) = right else { return Err(Errored(LoxError::with_message(invalid_operands_message))); };

        Ok(Expression::LiteralNumber(op(*left_num, *right_num)?))
    }

    fn comparison<N, S, B>(
        &self,
        left: &Expression,
        right: &Expression,
        n: N,
        s: S,
        b: B,
    ) -> Outcome<Expression>
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

                let Expression::LiteralNumber(right_num) = right else { return Err(Errored(LoxError::with_message("Cannot compare unlike types"))); };

                Ok(Expression::LiteralBoolean(n(*left_num, *right_num)))
            }
            Expression::LiteralString(left_string) => {
                if right == &Expression::Nil {
                    return Ok(Expression::LiteralBoolean(false));
                }

                let Expression::LiteralString(right_string) = right else { return Err(Errored(LoxError::with_message("Cannot compare unlike types"))); };

                Ok(Expression::LiteralBoolean(s(left_string, right_string)))
            }
            Expression::LiteralBoolean(left_bool) => {
                if right == &Expression::Nil {
                    return Ok(Expression::LiteralBoolean(false));
                }

                let Expression::LiteralBoolean(right_bool) = right else { return Err(Errored(LoxError::with_message("Cannot compare unlike types"))); };

                Ok(Expression::LiteralBoolean(b(*left_bool, *right_bool)))
            }
            Expression::Nil => {
                if right == &Expression::Nil {
                    Ok(Expression::LiteralBoolean(true))
                } else {
                    Ok(Expression::LiteralBoolean(false))
                }
            }
            _ => Err(Errored(LoxError::with_message("Invalid expression for comparison"))),
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

    pub fn resolve(&mut self, expression_id: u16, depth: usize) {
        self.locals.insert(expression_id, depth);
    }

    pub fn current_environment(&self) -> Rc<RefCell<Environment>> {
        self.environments.front().unwrap().clone()
    }

    pub fn current_environment_mut(&mut self) -> Rc<RefCell<Environment>> {
        self.environments.front_mut().unwrap().clone()
    }

    pub fn global_environment(&self) -> Rc<RefCell<Environment>> {
        self.environments.back().unwrap().clone()
    }

    pub fn global_environment_mut(&mut self) -> Rc<RefCell<Environment>> {
        self.environments.back_mut().unwrap().clone()
    }

    pub fn get_callable(&self, ident: &str) -> Option<Rc<dyn Callable>> {
        for env in self.environments.iter().rev() {
            match env.borrow().get_callable(ident) {
                Some(callable) => return Some(callable),
                None => (),
            }
        }

        None
    }

    pub fn assign_variable(&mut self, ident: &str, expression: &Expression) -> Outcome<()> {
        for env in self.environments.iter_mut().rev() {

            let mut e = env.borrow_mut();

            if e.has_declared_variable(ident) == true {
                e.assign(ident, expression)?;
                break;
            }
        }

        Ok(())
    }

    pub fn get_variable(&self, ident: &str) -> Option<Expression> {
        for env in self.environments.iter().rev() {
            match env.borrow().get(ident) {
                Some(var) => return Some(var),
                None => (),
            }
        }

        None
    }

    pub fn get_variable_at(&self, ident: &str, depth: usize) -> Option<Expression> {
        for el in self.environments.iter().rev().skip(depth) {
            match el.borrow().get(ident) {
                None => (),
                Some(expr) => return Some(expr)
            }
        }

        None
    }

    pub fn assign_variable_at(&mut self, ident: &str, expression: &Expression, depth: usize) -> Outcome<()> {
        for el in self.environments.iter_mut().rev().skip(depth) {

            let mut e = el.borrow_mut();

            if e.has_declared_variable(ident) == true {
                e.assign(ident, expression)?
            }
        }

        Ok(())
    }

    pub fn get_callable_at(&self, ident: &str, depth: usize) -> Option<Rc<dyn Callable>> {
        for el in self.environments.iter().rev().skip(depth) {
            match el.borrow().get_callable(ident) {
                None => (),
                Some(callable) => return Some(callable)
            }
        }

        None
    }


}
