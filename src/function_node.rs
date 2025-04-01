use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use regex::Regex;
use crate::ast_node::{ASTNode, Namespace, VarNode};
use crate::compiler_error::{raise_compiler_error, CompilerError};
use crate::compiler_error::CompilerError::UndefinedType;
use crate::data_type::Type;
use crate::line::Line;

pub struct Function {
    ret_type: Rc<Type>,
    name: Rc<String>,
    pub param_count: usize,
    pub namespace: Rc<RefCell<Namespace>>,
    body: Vec<Box<dyn ASTNode>>
}

const PAREN_CLOSE_TOKEN: &str = ")";
const PARAM_DELIMITER: &str = ",";

fn assert_unique_var(var_name: Rc<String>, line: &Line, namespace: Rc<RefCell<Namespace>>) {
    if namespace.borrow().contains_var(var_name.clone()) {
        raise_compiler_error(CompilerError::SymbolAlreadyDefined(line.line_num, var_name.to_string()))
    }
}

fn assert_valid_symbol(symbol: Rc<String>, line: &Line) {
    let valid_symbol_regex: Regex = Regex::new("^\\w+$").unwrap();

    if !valid_symbol_regex.is_match(&symbol) {
        raise_compiler_error(CompilerError::InvalidSymbol(line.line_num, symbol.to_string()));
    }
}

fn assert_has_min_tokens(min_tokens: usize, line: &Line) {
    if line.end - line.start < min_tokens {
        raise_compiler_error(CompilerError::InvalidDefinition(line.line_num));
    }
}

fn assert_correct_delimiter(i: usize, line: &Line) {
    let delimiter = if i == line.len() - 1 {
        PAREN_CLOSE_TOKEN
    } else {
        PARAM_DELIMITER
    };

    let token = line[i].clone();
    if *token != delimiter {
        raise_compiler_error(CompilerError::UnexpectedToken(line.line_num, token.to_string()));
    }
}

fn assert_valid_type(type_name: Rc<String>, type_opt: Option<&Rc<Type>>, line: &Line) {
    if type_opt.is_none() {
        raise_compiler_error(UndefinedType(line.line_num, type_name.to_string()));
    }
}

impl ASTNode for Function {
    fn get_type(&self) -> Rc<Type> {
        self.ret_type.clone()
    }
}

impl Function {
    fn new(ret_type: Rc<Type>, name: Rc<String>) -> Self {
        Self {
            ret_type,
            name,
            param_count: 0,
            namespace: Rc::new(RefCell::new(Namespace::new())),
            body: Vec::new(),
        }
    }

    fn add_param(&mut self, var: VarNode) {
        self.add_var(var);
        self.param_count += 1;
    }

    fn add_var(&mut self, var: VarNode) {
        self.namespace.borrow_mut().add_var(var);
    }

    fn parse_func_params(&mut self,
                         line: &Line,
                         types: &HashMap<String, Rc<Type>>
    ) {
        const PARAM_START_INDEX: usize = 3;

        if line[PARAM_START_INDEX].as_str() != PAREN_CLOSE_TOKEN {
            return;
        }

        let mut i = PARAM_START_INDEX;
        while i < line.len() {
            const MIN_TOKENS_REMAINING: usize = 3;
            assert_has_min_tokens(i + MIN_TOKENS_REMAINING, line);

            let param_type_name = line[i].clone();
            let param_type_opt = types.get(param_type_name.as_str());
            assert_valid_type(param_type_name, param_type_opt, line);
            let param_type = param_type_opt.unwrap();

            i += 1;
            let param_name = line[i].clone();
            assert_valid_symbol(param_name.clone(), line);

            i += 1;
            assert_correct_delimiter(i, line);

            assert_unique_var(param_name.clone(), &line, self.namespace.clone());
            self.add_param(VarNode::new(param_type.clone(), param_name));
            i += 1;
        }
    }

    fn create_func_def_node(ret_type: Rc<Type>,
                            line: &Line,
                            types: &HashMap<String, Rc<Type>>
    ) -> Self {
        const NAME_INDEX: usize = 1;

        let mut func_node = Self::new(ret_type, line[NAME_INDEX].clone());
        func_node.parse_func_params(line, types);

        func_node
    }

    pub fn from(line: &Line,
                types: &HashMap<String, Rc<Type>>
    ) -> Option<Function> {
        const MIN_DEF_TOKENS: usize = 4;
        assert_has_min_tokens(MIN_DEF_TOKENS, line);

        match types.get(line[0].as_ref()).cloned() {
            None => { None }
            Some(ret_type) => {
                Some(Self::create_func_def_node(ret_type, line, types))
                // parse body
            }
        }
    }
}