use std::collections::HashMap;
use crate::data_type::Type;

pub trait ASTNode {
    fn get_type(&self) -> &Type;
}

#[derive(Copy, Clone)]
pub struct VarNode<'a> {
    data_type: &'a Type,
    name: &'a str
}

impl<'a> ASTNode for VarNode<'a> {
    fn get_type(&self) -> &'a Type {
        &self.data_type
    }
}

impl<'a> VarNode<'a> {
    pub fn new(data_type: &'a Type, name: &'a str) -> Self {
        Self { data_type, name }
    }
}

pub struct Literal<'a> {
    data_type: &'a Type,
    value: String
}

pub struct Function<'a> {
    ret_type: &'a Type,
    name: &'a String,
    pub param_count: usize,
    pub namespace: Namespace<'a>,
    body: Vec<Box<dyn ASTNode>>
}

impl<'a> Function<'a> {
    pub fn new(ret_type: &'a Type, name: &'a String) -> Self {
        Self { ret_type, name, param_count: 0, namespace: Namespace::new(), body: Vec::new() }
    }

    pub fn add_param(&mut self, var: VarNode<'a>) {
        self.add_var(var);
        self.param_count += 1;
    }

    pub fn add_var(&mut self, var: VarNode<'a>) {
        self.namespace.add_var(var);
    }
}

pub struct BinaryOperator<'a> {
    expr_type: &'a Type,
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>
}

pub struct Namespace<'a> {
    vars: HashMap<&'a str, VarNode<'a>>,
    parent: Option<&'a Namespace<'a>>
}

impl<'a> Namespace<'a> {
    pub fn new() -> Self {
        Self { vars: HashMap::new(), parent: None }
    }

    pub fn contains_var(&self, var_name: &str) -> bool {
        self.vars.contains_key(var_name)
    }

    pub fn add_var(&mut self, var_node: VarNode<'a>) {
        self.vars.insert(var_node.name, var_node);
    }
}
