use std::collections::HashMap;
use crate::data_type::Type;

pub trait ASTNode {
    fn get_type(&self) -> &Type;
}

pub struct Var<'a> {
    data_type: &'a Type,
    name: String
}

pub struct Literal<'a> {
    data_type: &'a Type,
    value: String
}

pub struct Function<'a> {
    ret_type: &'a Type,
    name: String,
    param_count: usize,
    body: Vec<Box<dyn ASTNode>>
}

pub struct BinaryOperator<'a> {
    expr_type: &'a Type,
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>
}

pub struct Namespace<'a> {
    vars: HashMap<String, &'a Var<'a>>,
    parent: Box<Namespace<'a>>
}

impl<'a> ASTNode for Var<'a> {
    fn get_type(&self) -> &'a Type {
        &self.data_type
    }
}

impl<'a> Var<'a> {
    pub(crate) fn new(data_type: &'a Type, name: &String) -> Self {
        Self { data_type, name: name.to_string() }
    }
}

impl<'a> Namespace<'a> {
    pub fn contains_var(&self, var_name: &String) -> bool {
        self.vars.contains_key(var_name)
    }

    pub fn add_var(&mut self, var_node: &'a Var) {
        self.vars.insert(var_node.name.to_string(), var_node);
    }
}
