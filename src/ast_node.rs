use crate::data_type::Type;

pub trait AstNode {
    fn get_type(&self) -> Type;
}

pub struct Var {
    data_type: Type,
    name: String
}

pub struct Literal {
    data_type: Type,
    value: String
}

pub struct Function {
    ret_type: Type,
    name: String,
    param_count: usize,
    body: Vec<Box<dyn AstNode>>
}

pub struct BinaryOperator {
    expr_type: Type,
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>
}

pub struct Namespace {
    vars: Vec<Var>,
    parent: Box<Namespace>
}

impl AstNode for Var {
    fn get_type(&self) -> Type {
        self.data_type
    }
}
