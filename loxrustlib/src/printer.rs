use crate::expr::{
    Assign, Binary, Call, Comma, Expression, Get, Grouping, Literal, Logical, Set, Super, This,
    Unary, Variable, Visitor,
};

pub struct PrettyPrinter {}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter {}
    }

    pub fn print(&self, expr: &Box<dyn Expression>) {
        expr.accept(self);
        print!("\n");
    }
}

impl Visitor for PrettyPrinter {
    fn visit_assign(&self, assign: &Assign) {
        print!("(assign {} ", assign.name);
        assign.expr.accept(self);
        print!(")")
    }

    fn visit_binary(&self, binary: &Binary) {
        print!("(binary ");
        binary.left.accept(self);
        print!(" {} ", binary.operator);
        binary.right.accept(self);
        print!(")");
    }

    fn visit_unary(&self, unary: &Unary) {
        print!("(unary {} ", unary.operator);
        unary.right.accept(self);
        print!(")");
    }

    fn visit_call(&self, call: &Call) {
        print!("(call ");
        call.callee.accept(self);
        print!(" {} ", call.paren);
        call.arguments.iter().for_each(|expr| {
            print!(", ");

            expr.accept(self)
        });
        print!(")")
    }

    fn visit_get(&self, get: &Get) {
        print!("(get ");
        get.expr.accept(self);
        print!(" {} ", get.name);
        print!(")");
    }

    fn visit_grouping(&self, grouping: &Grouping) {
        print!("(grouping ");
        grouping.expression.accept(self);
        print!(")");
    }

    fn visit_literal(&self, literal: &Literal) {
        print!("(literal {})", literal.literal);
    }

    fn visit_logical(&self, logical: &Logical) {
        print!("(logical ");
        logical.left.accept(self);
        print!(" {} ", logical.operator);
        logical.right.accept(self);
        print!(")");
    }

    fn visit_set(&self, set: &Set) {
        print!("(set ");
        set.object.accept(self);
        print!(" {} ", set.name);
        set.value.accept(self);
        print!(")");
    }

    fn visit_super(&self, super_expr: &Super) {
        print!("(super {} {})", super_expr.keyword, super_expr.method);
    }

    fn visit_this(&self, this: &This) {
        print!("(this {})", this.keyword);
    }

    fn visit_variable(&self, variable: &Variable) {
        print!("(variable {})", variable.name);
    }

    fn visit_comma(&self, comma: &Comma) {
        print!("(comma ");
        comma.expressions.iter().for_each(|expr| {
            print!(", ");

            expr.accept(self)
        });
        print!(")");
    }
}
