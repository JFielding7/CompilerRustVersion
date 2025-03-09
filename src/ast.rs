use std::collections::HashMap;
use regex::Regex;
use crate::ast_node::{ASTNode, Function, Namespace, VarNode};
use crate::data_type::Type;
use crate::line::{Line, LineIterator};
use crate::compiler_error::{raise_compiler_error, CompilerError};

const ASSIGNMENT_TOKEN: &str = "=";
const PAREN_OPEN_TOKEN: &str = "(";
const PAREN_CLOSE_TOKEN: &str = ")";
const PARAM_DELIMITER: &str = ",";

fn assert_unique_var(var_name: &String, line: &Line, namespace: &Namespace) {
    if namespace.contains_var(var_name) {
        raise_compiler_error(CompilerError::SymbolAlreadyDefined(line.line_num, var_name.to_string()))
    }
}

fn assert_valid_symbol(symbol: &String, line: &Line) {
    let valid_symbol_regex: Regex = Regex::new("^\\w+$").unwrap();

    if !valid_symbol_regex.is_match(symbol) {
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

    let token = &line.tokens[i];
    if token != delimiter {
        raise_compiler_error(CompilerError::InvalidToken(line.line_num, token.to_string()));
    }
}

fn var_def_node<'a>(data_type: &'a Type, line: &Line<'a>, namespace: &mut Namespace<'a>) -> VarNode<'a> {
    const VAR_NAME_INDEX: usize = 1;

    let var_name = &line.tokens[1];
    assert_unique_var(var_name, line, namespace);

    let var_node = VarNode::new(data_type, var_name);
    namespace.add_var(var_node);
    var_node
}

fn function_def_node<'a>(ret_type: &'a Type, line: &Line<'a>, types: &'a HashMap<String, Type>) -> Function<'a> {
    const NAME_INDEX: usize = 1;
    const PARAM_START_INDEX: usize = 3;

    let mut func_node = Function::new(ret_type, &line.tokens[NAME_INDEX]);
    let mut i = PARAM_START_INDEX;

    if line.tokens[PARAM_START_INDEX] != PAREN_CLOSE_TOKEN {
        while i < line.len() {
            const MIN_TOKENS_REMAINING: usize = 3;
            assert_has_min_tokens(i + MIN_TOKENS_REMAINING, line);

            let param_type_name = &line.tokens[i];
            let param_type = types.get(param_type_name).unwrap();

            i += 1;
            let param_name = &line.tokens[i];
            assert_valid_symbol(param_name, line);

            i += 1;
            assert_correct_delimiter(i, line);

            assert_unique_var(param_name, &line, &func_node.namespace);
            func_node.add_param(VarNode::new(param_type, param_name));
            i += 1;
        }
    }

    func_node
}

fn symbol_definition<'a>(data_type: &'a Type, curr_line: &Line<'a>, namespaces: &'a mut Vec<Namespace<'a>>) -> Result<Box<dyn ASTNode + 'a>, CompilerError> {
    let symbol = &curr_line.tokens[curr_line.start + 1];
    assert_valid_symbol(symbol, curr_line);

    let token = &curr_line.tokens[curr_line.start + 2];
    match token.as_str() {
        ASSIGNMENT_TOKEN => {
            Ok(Box::new(var_def_node(data_type, curr_line, namespaces.last_mut().unwrap())))
        }
        _ => {
            Err(CompilerError::InvalidDefinition(curr_line.line_num))
        }
    }
}

fn create_ast_node<'a>(curr_line: &Line<'a>, types: &'a HashMap<String, Type>, namespaces: &'a mut Vec<Namespace<'a>>) -> Result<Box<dyn ASTNode + 'a>, CompilerError> {
    let start_token = curr_line.get_token(0);

    if let Some(data_type) = types.get(start_token) {
        const MIN_DEF_TOKENS: usize = 4;
        assert_has_min_tokens(MIN_DEF_TOKENS, curr_line);
        return symbol_definition(data_type, curr_line, namespaces);
    }

    Err(CompilerError::InvalidDefinition(curr_line.line_num))
}

fn generate_ast(filename: &String, tokens: Vec<String>, types: &mut HashMap<String, Type>) {
    let mut namespaces: Vec<Namespace> = vec![];

    let mut line_iter = LineIterator::new(filename, tokens);

    while let Some(curr_line) = line_iter.next() {
        if curr_line.start < curr_line.end {
            // create_ast_node(&curr_line, types, &mut namespaces);
        }

        namespaces.push(Namespace::new());
    }
}
