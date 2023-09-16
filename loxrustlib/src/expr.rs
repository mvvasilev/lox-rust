use crate::token::Token;

pub trait Visitor {
    fn visit_assign(&self, assign: &Assign);
    fn visit_binary(&self, binary: &Binary);
    fn visit_unary(&self, unary: &Unary);
    fn visit_call(&self, call: &Call);
    fn visit_get(&self, get: &Get);
    fn visit_grouping(&self, grouping: &Grouping);
    fn visit_literal(&self, literal: &Literal);
    fn visit_logical(&self, logical: &Logical);
    fn visit_set(&self, set: &Set);
    fn visit_super(&self, super_expr: &Super);
    fn visit_this(&self, this: &This);
    fn visit_variable(&self, variable: &Variable);
    fn visit_comma(&self, comma: &Comma);
}

pub trait Expression: std::fmt::Debug {
    fn accept(&self, visitor: &dyn Visitor);
}

#[derive(Debug)]
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
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_assign(self)
    }
}

#[derive(Debug)]
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
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_binary(self)
    }
}

#[derive(Debug)]
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
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_call(self)
    }
}

#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<dyn Expression>,
}

impl Grouping {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Grouping { expression }
    }
}

impl Expression for Grouping {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_grouping(self)
    }
}

#[derive(Debug)]
pub struct Literal {
    pub literal: Token,
}

impl Literal {
    pub fn new(literal: Token) -> Self {
        Literal { literal }
    }
}

impl Expression for Literal {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_literal(self)
    }
}

#[derive(Debug)]
pub struct Logical {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Expression for Logical {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_logical(self)
    }
}

#[derive(Debug)]
pub struct Get {
    pub expr: Box<dyn Expression>,
    pub name: Token,
}

impl Expression for Get {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_get(self)
    }
}

#[derive(Debug)]
pub struct Set {
    pub object: Box<dyn Expression>,
    pub name: Token,
    pub value: Box<dyn Expression>,
}

impl Expression for Set {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_set(self)
    }
}

#[derive(Debug)]
pub struct Super {
    pub keyword: Token,
    pub method: Token,
}

impl Expression for Super {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_super(self)
    }
}

#[derive(Debug)]
pub struct This {
    pub keyword: Token,
}

impl Expression for This {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_this(self)
    }
}

#[derive(Debug)]
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
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_unary(self)
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: Token,
}

impl Expression for Variable {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_variable(self)
    }
}

#[derive(Debug)]
pub struct Comma {
    pub expressions: Vec<Box<dyn Expression>>,
}
impl Comma {
    pub fn new(expressions: Vec<Box<dyn Expression>>) -> Self {
        Self { expressions }
    }
}

impl Expression for Comma {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_comma(self);
    }
}
