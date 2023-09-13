use crate::token::Token;

pub trait Visitor {
    fn visit_assign(&mut self, assign: &mut Assign);
    fn visit_binary(&mut self, binary: &mut Binary);
    fn visit_unary(&mut self, unary: &mut Unary);
    fn visit_call(&mut self, call: &mut Call);
    fn visit_get(&mut self, get: &mut Get);
    fn visit_grouping(&mut self, grouping: &mut Grouping);
    fn visit_literal(&mut self, literal: &mut Literal);
    fn visit_logical(&mut self, logical: &mut Logical);
    fn visit_set(&mut self, set: &mut Set);
    fn visit_super(&mut self, super_expr: &mut Super);
    fn visit_this(&mut self, this: &mut This);
    fn visit_variable(&mut self, variable: &mut Variable);
}

pub struct PrettyPrinter {}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter {}
    }

    pub fn print(&mut self, expr: &mut dyn Expression) {
        return expr.accept(self);
    }
}

impl Visitor for PrettyPrinter {
    fn visit_assign(&mut self, assign: &mut Assign) {
        print!("(assign {} ", assign.name);
        assign.expr.accept(self);
        print!(")")
    }

    fn visit_binary(&mut self, binary: &mut Binary) {
        print!("(binary ");
        binary.left.accept(self);
        print!(" {} ", binary.operator);
        binary.right.accept(self);
        print!(")");
    }

    fn visit_unary(&mut self, unary: &mut Unary) {
        print!("(unary {} ", unary.operator);
        unary.right.accept(self);
        print!(")");
    }

    fn visit_call(&mut self, call: &mut Call) {
        print!("(call ");
        call.callee.accept(self);
        print!(" {} ", call.paren);
        call.arguments.iter_mut().map(|expr| expr.accept(self));
        print!(")")
    }

    fn visit_get(&mut self, get: &mut Get) {
        print!("(get ");
        get.expr.accept(self);
        print!(" {} ", get.name);
        print!(")");
    }

    fn visit_grouping(&mut self, grouping: &mut Grouping) {
        print!("(grouping ");
        grouping.expression.accept(self);
        print!(")");
    }

    fn visit_literal(&mut self, literal: &mut Literal) {
        print!("(literal {})", literal.literal);
    }

    fn visit_logical(&mut self, logical: &mut Logical) {
        print!("(logical ");
        logical.left.accept(self);
        print!(" {} ", logical.operator);
        logical.right.accept(self);
        print!(")");
    }

    fn visit_set(&mut self, set: &mut Set) {
        print!("(set ");
        set.object.accept(self);
        print!(" {} ", set.name);
        set.value.accept(self);
        print!(")");
    }

    fn visit_super(&mut self, super_expr: &mut Super) {
        print!("(super {} {})", super_expr.keyword, super_expr.method);
    }

    fn visit_this(&mut self, this: &mut This) {
        print!("(this {})", this.keyword);
    }

    fn visit_variable(&mut self, variable: &mut Variable) {
        print!("(variable {})", variable.name);
    }
}

pub trait Expression {
    fn accept(&mut self, visitor: &mut dyn Visitor);
}

pub struct Assign {
    pub name: Token,
    pub expr: Box<dyn Expression>,
}

impl Assign {
    pub fn new(name: Token, expr: Box<dyn Expression>) -> Self {
        Assign { name, expr }
    }
}

impl Expression for Assign {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_assign(self)
    }
}

pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Binary {
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl Expression for Binary {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_binary(self)
    }
}

pub struct Call {
    pub callee: Box<dyn Expression>,
    pub paren: Token,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Call {
    pub fn new(
        callee: Box<dyn Expression>,
        paren: Token,
        arguments: Vec<Box<dyn Expression>>,
    ) -> Self {
        Call {
            callee,
            paren,
            arguments,
        }
    }
}

impl Expression for Call {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_call(self)
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expression>,
}

impl Grouping {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Grouping { expression }
    }
}

impl Expression for Grouping {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_grouping(self)
    }
}

pub struct Literal {
    pub literal: Token,
}

impl Literal {
    pub fn new(literal: Token) -> Self {
        Literal { literal }
    }
}

impl Expression for Literal {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_literal(self)
    }
}

pub struct Logical {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Expression for Logical {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_logical(self)
    }
}

pub struct Get {
    pub expr: Box<dyn Expression>,
    pub name: Token,
}

impl Expression for Get {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_get(self)
    }
}

pub struct Set {
    pub object: Box<dyn Expression>,
    pub name: Token,
    pub value: Box<dyn Expression>,
}

impl Expression for Set {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_set(self)
    }
}

pub struct Super {
    pub keyword: Token,
    pub method: Token,
}

impl Expression for Super {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_super(self)
    }
}

pub struct This {
    pub keyword: Token,
}

impl Expression for This {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_this(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expression>) -> Self {
        Unary { operator, right }
    }
}

impl Expression for Unary {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_unary(self)
    }
}

pub struct Variable {
    pub name: Token,
}

impl Expression for Variable {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_variable(self)
    }
}
