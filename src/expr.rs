use std::rc::Rc;

use crate::token::Token;

trait Visitor<R> {
    fn visit_assign(&mut self, assign: &mut Assign) -> R;
    fn visit_binary(&mut self, binary: &mut Binary) -> R;
    fn visit_unary(&mut self, binary: &mut Unary) -> R;
    fn visit_call(&mut self, call: &mut Call) -> R;
    fn visit_get(&mut self, get: &mut Get) -> R;
    fn visit_grouping(&mut self, grouping: &mut Grouping) -> R;
    fn visit_literal(&mut self, literal: &mut Literal) -> R;
    fn visit_logical(&mut self, logical: &mut Logical) -> R;
    fn visit_set(&mut self, set: &mut Set) -> R;
    fn visit_super(&mut self, super_expr: &mut Super) -> R;
    fn visit_this(&mut self, this: &mut This) -> R;
    fn visit_variable(&mut self, variable: &mut Variable) -> R;
}

struct PrettyPrinter {}

impl Visitor<String> for PrettyPrinter {
    fn visit_assign(&mut self, assign: &mut Assign) -> String {
        
    }

    fn visit_binary(&mut self, binary: &mut Binary) -> String {
        todo!()
    }

    fn visit_unary(&mut self, binary: &mut Unary) -> String {
        todo!()
    }

    fn visit_call(&mut self, call: &mut Call) -> String {
        todo!()
    }

    fn visit_get(&mut self, get: &mut Get) -> String {
        todo!()
    }

    fn visit_grouping(&mut self, grouping: &mut Grouping) -> String {
        todo!()
    }

    fn visit_literal(&mut self, literal: &mut Literal) -> String {
        todo!()
    }

    fn visit_logical(&mut self, logical: &mut Logical) -> String {
        todo!()
    }

    fn visit_set(&mut self, set: &mut Set) -> String {
        todo!()
    }

    fn visit_super(&mut self, super_expr: &mut Super) -> String {
        todo!()
    }

    fn visit_this(&mut self, this: &mut This) -> String {
        todo!()
    }

    fn visit_variable(&mut self, variable: &mut Variable) -> String {
        todo!()
    }
}

trait Expression {}

struct Assign {
    name: Token,
    expr: Rc<dyn Expression>,
}

impl Expression for Assign {}

struct Binary {
    left: Rc<dyn Expression>,
    operator: Token,
    right: Rc<dyn Expression>,
}

impl Expression for Binary {}

struct Call {
    callee: Rc<dyn Expression>,
    paren: Token,
    arguments: Vec<Rc<dyn Expression>>,
}

impl Expression for Call {}

struct Grouping {
    expression: Rc<dyn Expression>,
}

impl Expression for Grouping {}

struct Literal {
    num: Token,
}

impl Expression for Literal {}

struct Logical {
    left: Rc<dyn Expression>,
    operator: Token,
    right: Rc<dyn Expression>,
}

impl Expression for Logical {}

struct Get {
    expr: Rc<dyn Expression>,
    name: Token,
}

impl Expression for Get {}

struct Set {
    object: Rc<dyn Expression>,
    name: Token,
    value: Rc<dyn Expression>,
}

impl Expression for Set {}

struct Super {
    keyword: Token,
    method: Token,
}

impl Expression for Super {}

struct This {
    keyword: Token,
}

impl Expression for This {}

struct Unary {
    operator: Token,
    right: Rc<dyn Expression>,
}

impl Expression for Unary {}

struct Variable {
    name: Token,
}

impl Expression for Variable {}
