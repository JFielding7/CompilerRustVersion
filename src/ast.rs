use crate::ast_node::{AstNode, Namespace};
use crate::line_iterator::{Line, LineIterator};

fn create_ast_node(curr_line: &Line, namespaces: &Vec<Box<dyn AstNode>>) {

}

fn generate_ast(filename: &String, tokens: Vec<String>) {
    let namespaces: Vec<Namespace> = vec![];

    let mut line_iter = LineIterator::new(filename, &tokens);

    while let Some(curr_line) = line_iter.next() {
        if curr_line.start < curr_line.end {

        }
    }
}
// vec namespaces = vec_new();
//
// line_iterator iter;
// init_line_iterator(&iter, filename, tokenv);
//
// line *curr_line = next_line(&iter);
// while (curr_line != NULL) {
//
// if (vec_len(namespaces) > 0) {
// namespace *ns = vec_get(namespaces, 0);
// printf("curr len: %lu\n", vec_len(ns->vars));
// }
//
// if (curr_line->start < curr_line->end) {
// ast_node *node = create_ast_node(tokenv, curr_line, namespaces);
// puts("Node:");
// ast_tree_print(node);
// }
// curr_line = next_line(&iter);
// }
//
// vec_free(namespaces);
//
// return NULL;
// }