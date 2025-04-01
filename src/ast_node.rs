use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use crate::data_type::Type;
use crate::function_node::Function;

pub struct SourceFile {
    functions: Vec<Function>
}

impl SourceFile {
    pub fn new() -> Self {
        Self { functions: Vec::new() }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }
}

pub trait ASTNode {
    fn get_type(&self) -> Rc<Type>;
}

#[derive(Debug)]
pub struct VarNode {
    data_type: Rc<Type>,
    name: Rc<String>,
}

impl ASTNode for VarNode {
    fn get_type(&self) -> Rc<Type> {
        self.data_type.clone()
    }
}

impl VarNode {
    pub fn new(data_type: Rc<Type>, name: Rc<String>) -> Self {
        Self { data_type, name }
    }
}

impl Clone for VarNode {
    fn clone(&self) -> Self {
        Self::new(self.data_type.clone(), self.name.clone())
    }
}

pub struct Literal {
    data_type: Rc<Type>,
    value: Rc<String>,
}

impl ASTNode for Literal {
    fn get_type(&self) -> Rc<Type> {
        self.data_type.clone()
    }
}

impl Literal {
    pub fn new(data_type: Rc<Type>, value: Rc<String>) -> Self {
        Self { data_type, value }
    }
}

pub struct BinaryOperator {
    data_type: Rc<Type>,
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl BinaryOperator {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        let l = left.get_type();
        Self { data_type: l, left, right }
    }
}

impl ASTNode for BinaryOperator {
    fn get_type(&self) -> Rc<Type> {
        self.data_type.clone()
    }
}

#[derive(Debug)]
pub struct Namespace {
    vars: HashMap<Rc<String>, VarNode>,
    pub parent: Option<Rc<RefCell<Namespace>>>
}

impl Namespace {
    pub fn new() -> Self {
        Self { vars: HashMap::new(), parent: None }
    }

    pub fn contains_var(&self, var_name: Rc<String>) -> bool {
        self.vars.contains_key(&var_name)
    }

    pub fn get_var(&self, var_name: &String) -> Option<VarNode> {
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

    pub fn add_var(&mut self, var_node: VarNode) {
        self.vars.insert(var_node.name.clone(), var_node);
    }
}
