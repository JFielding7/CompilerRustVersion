use phf::phf_map;
use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;
use std::string::ToString;
use crate::ast_node::{ASTNode, BinaryOperator, Literal, Namespace};
use crate::compiler_error::{raise_compiler_error, CompilerError};
use crate::compiler_error::CompilerError::{InvalidAssignment, InvalidSymbol, MismatchedParentheses};
use crate::data_type::{get_literal_type, Type};
use crate::line::Line;

const PAREN_OPEN: &str = "(";
const PAREN_CLOSE: &str = ")";

struct Expression<'a> {
    tokens: &'a [String],
    token_index: usize,
    line: &'a Line<'a>,
    op_group_index: usize,
    paren_matches: &'a mut [usize],
    namespace: Rc<Namespace<'a>>,
    types: &'a HashMap<String, Type>,
}

impl<'a> Expression<'a> {
    fn new(line: &'a Line,
           start: usize,
           end: usize,
           paren_matches: &mut [usize],
           types: &'a HashMap<String, Type>,
           namespace: Rc<Namespace<'a>>
    ) -> Self {
        Self {
            tokens: &line.tokens[start..end],
            token_index: 0,
            line,
            op_group_index: 0,
            paren_matches,
            namespace,
            types,
        }
    }

    fn len(&self) -> usize {
        self.tokens.len()
    }

    fn last(&self) -> &String {
        &self[self.len() - 1]
    }
}

impl Index<usize> for Expression<'_> {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}

const OPERATOR_GROUPS: usize = 2;
static OPERATORS: [phf::Map<&'static str, fn(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError>>; OPERATOR_GROUPS] = [
    phf_map! {
        "+" => add_parser,
        "-" => sub_parser,
    },
    phf_map! {
        "*" => mul_parser,
        "/" => div_parser,
        "%" => mod_parser,
    }
];

fn get_operator_parser<'a>(
    expression: &Expression
) -> Option<fn(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError>> {
    for (&token, &operator) in &OPERATORS[expression.op_group_index] {
        if token == expression[expression.token_index] {
            return Some(operator);
        }
    }

    None
}

fn binary_operation_parser(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {

}

fn add_parser(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Ok(Box::new(()))
}

fn sub_parser(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Ok(Box::new(()))
}

fn mul_parser(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Ok(Box::new(()))
}

fn div_parser(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Ok(Box::new(()))
}

fn mod_parser(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Ok(Box::new(()))
}

fn assignment_parser(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    if expression.token_index != 1 {
        return Err(InvalidAssignment(expression.line.line_num));
    }

    let var_node_opt = expression.namespace.borrow().get_var(&expression.tokens[0]);
    if var_node_opt.is_none() {
        return Err(InvalidSymbol(expression.line.line_num, expression.tokens[0].to_string()));
    }
    let var_node = var_node_opt.unwrap();

    // let mut assignment_expr = expression.clone();
    // assignment_expr.tokens = expression.tokens;
    let value = parse_expression(expression);
    match value {
        Ok(node) => {
            Ok(Box::new(BinaryOperator::new(var_node.get_type(), Box::new(var_node), node)))
        }
        err => err,
    }
}

fn parse_parenthetical_expression(mut expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    expression.tokens = &expression.tokens[1..expression.tokens.len() - 1];
    expression.op_group_index = 0;
    parse_expression(expression)
}

fn parse_value(expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    let token = &expression.tokens[expression.token_index];

    if let Some(literal_type) = get_literal_type(expression.types, token) {
        return Ok(Box::new(Literal::new(literal_type, token)));
    }

    if let Some(var_node) = expression.namespace.borrow().get_var(token) {
        return Ok(Box::new(var_node));
    }

    Err(InvalidSymbol(expression.line.line_num, token.to_string()))
}

fn parse_expression(mut expression: Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    if expression.len() == 1 {
        return parse_value(expression);
    }

    if expression[0] == PAREN_OPEN && expression.last() == PAREN_CLOSE {
        return parse_parenthetical_expression(expression);
    }

    while expression.op_group_index < OPERATOR_GROUPS {
        expression.token_index = expression.len() - 1;
        loop {
            let token = &expression[expression.token_index];
            if token == PAREN_CLOSE {
                expression.token_index = expression.paren_matches[expression.token_index]
            } else {
                if let Some(operator) = get_operator_parser(&expression) {
                    return operator.parse_func(expression);
                }
            }

            if expression.token_index == 0 {
                break;
            }
            expression.token_index -= 1;
        }

        expression.op_group_index += 1;
    }

    // TODO: handle errors
    Err(CompilerError::InvalidDefinition(expression.line.line_num))
}

fn match_parens(expression: &mut Expression) {
    let mut open_parens = Vec::new();

    for (i, token) in expression.tokens.iter().enumerate() {
        if token == PAREN_OPEN {
            open_parens.push(i);
        } else if token == PAREN_CLOSE {
            if open_parens.len() == 0 {
                raise_compiler_error(MismatchedParentheses(expression.line.line_num));
            }
            expression.paren_matches[i] = open_parens.pop().unwrap();
        }
    }

    if open_parens.len() != 0 {
        raise_compiler_error(MismatchedParentheses(expression.line.line_num));
    }
}

fn expression_node(line: &Line,
                   start: usize,
                   end: usize,
                   types: &HashMap<String, Type>,
                   namespace: Rc<Namespace>
) -> Result<Box<dyn ASTNode>, CompilerError> {
    let mut paren_matches = vec![end - start];
    let mut expression = Expression::new(line, start, end, paren_matches.as_mut_slice(), types, namespace);
    match_parens(&mut expression);
    parse_expression(expression)
}
