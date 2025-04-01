use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use regex::Regex;
use crate::ast_node::{ASTNode, Namespace, SourceFile, VarNode};
use crate::data_type::{compile_native_types, Type};
use crate::line::{Line, LineIterator};
use crate::compiler_error::{raise_compiler_error, CompilerError};
use crate::compiler_error::CompilerError::UndefinedType;
use crate::function_node::Function;
use crate::tokenizer::tokenize_file;

const ASSIGNMENT_TOKEN: &str = "=";
const PAREN_OPEN_TOKEN: &str = "(";
const PAREN_CLOSE_TOKEN: &str = ")";
const PARAM_DELIMITER: &str = ",";

// fn var_def_node(data_type: Rc<Type>, line: &Line, namespace: Rc<RefCell<Namespace>>) -> VarNode {
//     const VAR_NAME_INDEX: usize = 1;
//
//     let var_name = line[1].clone();
//     assert_unique_var(var_name.clone(), line, namespace.clone());
//
//     let var_node = VarNode::new(data_type, var_name);
//     namespace.borrow_mut().add_var(var_node.clone());
//     var_node
// }

// fn symbol_definition(data_type: Rc<Type>,
//                          curr_line: &Line,
//                          types: &HashMap<String, Rc<Type>>,
//                          namespace: &mut Rc<RefCell<Namespace>>
// ) -> Result<Box<dyn ASTNode>, CompilerError> {
//     let symbol = curr_line[1].clone();
//     assert_valid_symbol(symbol, curr_line);
//
//     let token = &curr_line[2];
//     match token.as_str() {
//         ASSIGNMENT_TOKEN => {
//             Ok(Box::new(var_def_node(data_type, curr_line, namespace.clone())))
//         }
//         PAREN_OPEN_TOKEN => {
//             let mut func_node = function_def_node(data_type, curr_line, types);
//             func_node.namespace.borrow_mut().parent = Some(namespace.clone());
//             *namespace = func_node.namespace.clone();
//             Ok(Box::new(func_node))
//         }
//         _ => {
//             Err(CompilerError::InvalidDefinition(curr_line.line_num))
//         }
//     }
// }

fn create_ast_node(curr_line: &Line,
                       types: &HashMap<String, Rc<Type>>,
                       namespace: &mut Rc<RefCell<Namespace>>
) -> Result<Box<dyn ASTNode>, CompilerError> {
    let start_token = curr_line.get_token(0);



    Err(CompilerError::InvalidDefinition(curr_line.line_num))
}

fn update_namespace(line: &Line, prev_indent: usize, namespace: &mut Rc<RefCell<Namespace>>) {
    if line.indent > prev_indent + 1 {
        raise_compiler_error(CompilerError::IndentError(line.line_num));
    }

    for _ in line.indent..prev_indent {
        let parent_ns = namespace.borrow().parent.as_ref().unwrap().clone();
        *namespace = parent_ns;
    }
}

fn add_function_def_nodes(source_file: &mut SourceFile,
                          lines: &Vec<Line>,
                          types: &HashMap<String, Rc<Type>>
) {
    for line in lines {
        match Function::from(line, types) {
            Some(func_node) => {
                source_file.add_function(func_node);
            }
            None => {}
        }
    }
}

pub fn generate_ast(file_name: &String) {
    let tokens = tokenize_file(file_name).unwrap();
    let mut types = compile_native_types();

    let mut namespace = Rc::new(RefCell::new(Namespace::new()));
    let lines: Vec<Line> = LineIterator::new(file_name, &tokens).collect();
    let mut prev_indent = 0;

    for curr_line in lines {
        if curr_line.start < curr_line.end {
            update_namespace(&curr_line, prev_indent, &mut namespace);
            prev_indent = curr_line.indent;

            match create_ast_node(&curr_line, &mut types, &mut namespace) {
                Ok(node) => {
                    println!("{:?}", node.get_type());
                }
                Err(e) => raise_compiler_error(e),
            }
            println!("{:?}", curr_line.tokens);
        }
    }
}
