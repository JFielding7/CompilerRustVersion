use std::collections::HashMap;
use regex::Regex;
use crate::ast_node::{ASTNode, Namespace, Var};
use crate::data_type::Type;
use crate::line::{Line, LineIterator};
use crate::compiler_error::{raise_compiler_error, CompilerError};

const ASSIGNMENT_TOKEN: &str = "=";
const PAREN_OPEN_TOKEN: &str = "(";

fn assert_unique_var(var_name: &String, line: &Line, namespace: &Namespace) {
    if namespace.contains_var(var_name) {
        raise_compiler_error(CompilerError::SymbolAlreadyDefined(line.line_num, var_name.to_string()))
    }
}

/**
 * Creates a AST Node for a variable definition
 * @param tokenv Tokens
 * @param var_type Data type of the variable
 * @param curr_line Current Line
 * @param ns Namespace to add the variable in
 * @return ast_node*: node for this variable defintion
 */
fn var_def_node(data_type: &Type, line: &Line, namespace: &mut Namespace) -> Var {
    let var_name = &line.tokens[line.start + 1];
    let var_node = Var::new(data_type, var_name);

    assert_unique_var(var_name, line, namespace);
    namespace.add_var(&var_node);
    var_node
}

/**
 * Creates AST Node for a function definition
 * @param tokenv Tokens
 * @param ret_type Return type of the function
 * @param curr_line Current Line
 * @param namespaces Namespaces hierarchy
 * @return ast_node*: node for this function defintion
 */
static ast_node *function_def_node(vec tokenv, type *ret_type, line *curr_line, vec namespaces) {
ast_node *node = function_node_new(ret_type, vec_get(tokenv, curr_line->start + 1));
function_node *func_node = node->node;

size_t i = curr_line->start + PARAM_START;
i += i + 1 == curr_line->end && strcmp(vec_get(tokenv, i), PAREN_CLOSE) == 0; // check if no paramerters

while (i < curr_line->end) {
assert_has_min_tokens(PARAM_MIN_TOKENS, i, curr_line);

char *type_name = vec_get(tokenv, i++);
type *param_type = get_type(type_name);
assert_valid_type(type_name, param_type, curr_line);

char *param_name = vec_get(tokenv, i++);
assert_valid_symbol(param_name, curr_line);

assert_token_equals(vec_get(tokenv, i), i + 1 == curr_line->end ? PAREN_CLOSE : PARAM_SEP, curr_line);
i++;
assert_unique_var(param_name, &func_node->func_namespace, curr_line);

function_node_add_var(func_node, var_node_new(param_type, param_name));
func_node->param_count++;
}

vec_push(namespaces, &func_node->func_namespace);
return node;
}

/**
 * Creates a AST Node for a symbol definition
 * @param tokenv Tokens
 * @param symbol_type Data type of the variable
 * @param curr_line Current Line
 * @param namespaces Namespace to add the variable in
 * @return ast_node*: node for this variable defintion
 */
fn symbol_definition(data_type: &Type, curr_line: &Line, namespaces: &Vec<Namespace>) -> Result<Box<dyn ASTNode>, CompilerError> {
    let symbol = &curr_line.tokens[curr_line.start + 1];
    assert_valid_symbol(symbol, curr_line);

    let token = &curr_line.tokens[curr_line.start + 2];
    match token {
        ASSIGNMENT_TOKEN => {  }
        _ => {
            panic!("{}", CompilerError::InvalidDefinition(curr_line.line_num));
        }
    }
}


fn assert_valid_symbol(symbol: &String, curr_line: &Line) {
    const VALID_SYMBOL_REGEX: Regex = Regex::new("^\\w+$").unwrap();

    if !VALID_SYMBOL_REGEX.is_match(symbol) {
        panic!("{}", CompilerError::InvalidSymbol(curr_line.line_num, symbol.to_string()));
    }
}

fn assert_has_min_tokens(line: &Line) {
    const MIN_TOKENS: usize = 4;
    if line.end - line.start < MIN_TOKENS {
        panic!("{}", CompilerError::InvalidDefinition(line.line_num));
    }
}

fn create_ast_node(curr_line: &Line, types: HashMap<String, Type>, namespaces: &Vec<Namespace>) {
    let start_token = curr_line.get_token(0);

    if let Some(data_type) = types.get(start_token) {
        assert_has_min_tokens(curr_line);
        return symbol_definition(data_type, curr_line, namespaces);
    }
}

/*
char *token = vec_get(tokenv, curr_line->start);

    // printf("Cheking def %s %lu\n", token, curr_line->start);
    type *symbol_type = get_type(token);
    if (symbol_type != NULL) {
        // puts("DEF");
        assert_has_min_tokens(MIN_SYMBOL_DEF_LEN, curr_line->start, curr_line);
        return symbol_definition(tokenv, symbol_type, curr_line, namespaces);
    }

    puts("Parsing");
    namespace *ns = vec_get(namespaces, 0);
    printf("len: %lu\n", vec_len(ns->vars));
    return parse_expression(tokenv, curr_line, curr_line->start, curr_line->end, vec_peek_end(namespaces));
 */

fn generate_ast(filename: &String, tokens: Vec<String>) {
    let namespaces: Vec<Namespace> = vec![];

    let mut line_iter = LineIterator::new(filename, &tokens);

    while let Some(curr_line) = line_iter.next() {
        if curr_line.start < curr_line.end {

        }
    }
}