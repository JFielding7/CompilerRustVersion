use crate::ast_node::{AstNode, Namespace};
use crate::line::{Line, LineIterator};

fn create_ast_node(curr_line: &Line, namespaces: &Vec<Box<dyn AstNode>>) {
    let start_token = curr_line.get_token(0);


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