use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use crate::data_type::Type;

pub trait ASTNode {
    fn get_type(&self) -> Rc<Type>;
}

#[derive(Debug)]
pub struct VarNode<'a> {
    data_type: Rc<Type>,
    name: &'a str,
}

impl ASTNode for VarNode<'_> {
    fn get_type(&self) -> Rc<Type> {
        self.data_type.clone()
    }
}

impl<'a> VarNode<'a> {
    pub fn new(data_type: Rc<Type>, name: &'a str) -> Self {
        Self { data_type, name }
    }
}

impl<'a> Clone for VarNode<'a> {
    fn clone(&self) -> Self {
        VarNode::new(self.data_type.clone(), self.name)
    }
}

pub struct Literal<'a> {
    data_type: Rc<Type>,
    value: &'a str,
}

impl<'a> ASTNode for Literal<'a> {
    fn get_type(&self) -> Rc<Type> {
        self.data_type.clone()
    }
}

impl<'a> Literal<'a> {
    pub fn new(data_type: Rc<Type>, value: &'a str) -> Self {
        Self { data_type, value }
    }
}

pub struct Function<'a> {
    ret_type: Rc<Type>,
    name: &'a String,
    pub param_count: usize,
    pub namespace: Rc<RefCell<Namespace<'a>>>,
    body: Vec<Box<dyn ASTNode>>
}

impl<'a> ASTNode for Function<'a> {
    fn get_type(&self) -> Rc<Type> {
        self.ret_type.clone()
    }
}

impl<'a> Function<'a> {
    pub fn new(ret_type: Rc<Type>, name: &'a String) -> Self {
        Self {
            ret_type,
            name,
            param_count: 0,
            namespace: Rc::new(RefCell::new(Namespace::new())),
            body: Vec::new(),
        }
    }

    pub fn add_param(&mut self, var: VarNode<'a>) {
        self.add_var(var);
        self.param_count += 1;
    }

    pub fn add_var(&mut self, var: VarNode<'a>) {
        self.namespace.borrow_mut().add_var(var);
    }
}

pub struct BinaryOperator {
    expr_type: Rc<Type>,
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl BinaryOperator {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        Self { expr_type: left.get_type(), left, right }
    }
}

impl ASTNode for BinaryOperator {
    fn get_type(&self) -> Rc<Type> {
        self.expr_type.clone()
    }
}

#[derive(Debug)]
pub struct Namespace<'a> {
    vars: HashMap<&'a str, VarNode<'a>>,
    pub parent: Option<Rc<RefCell<Namespace<'a>>>>
}

impl<'a> Namespace<'a> {
    pub fn new() -> Self {
        Self { vars: HashMap::new(), parent: None }
    }

    pub fn contains_var(&self, var_name: &str) -> bool {
        self.vars.contains_key(var_name)
    }

    pub fn get_var(&self, var_name: &str) -> Option<VarNode> {
        if let Some(var) = self.vars.get(var_name) {
            return Some(var.clone())
        }

        let mut namespace_opt = self.parent.clone();
        while let Some(namespace) = namespace_opt {
            let curr_namespace =  namespace.borrow();
            if let Some(var) = curr_namespace.vars.get(var_name) {
                return Some(var.clone());
            }
            namespace_opt = curr_namespace.parent.clone();
        }

        None
    }

    pub fn add_var(&mut self, var_node: VarNode<'a>) {
        self.vars.insert(var_node.name, var_node);
    }
}
