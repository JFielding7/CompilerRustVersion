use phf::phf_map;
use std::collections::HashMap;
use std::env::var;
use std::ops::Index;
use std::rc::Rc;
use std::string::ToString;
use crate::ast_node::{ASTNode, BinaryOperator, Literal, Namespace};
use crate::compiler_error::{raise_compiler_error, CompilerError};
use crate::compiler_error::CompilerError::{BinaryOperatorTypeError, InvalidAssignment, InvalidExpression, InvalidSymbol, MismatchedParentheses};
use crate::data_type::{get_literal_type, Type};
use crate::line::Line;

const PAREN_OPEN: &str = "(";
const PAREN_CLOSE: &str = ")";

struct Expression<'a> {
    tokens: &'a [String],
    token_index: usize,
    line: &'a Line<'a>,
    op_group_index: usize,
    paren_matches: &'a [usize],
    types: &'a HashMap<String, Rc<Type>>,
    namespace: Rc<Namespace<'a>>,
}

impl<'a> Expression<'a> {
    fn new(line: &'a Line,
           start: usize,
           end: usize,
           types: &'a HashMap<String, Rc<Type>>,
           namespace: Rc<Namespace<'a>>
    ) -> Self {
        Self {
            tokens: &line.tokens[start..end],
            token_index: 0,
            line,
            op_group_index: 0,
            paren_matches: &[],
            types,
            namespace,
        }
    }

    fn sub_expression(&'a self, tokens: &'a [String], op_group_index: usize) -> Self {
        Expression {
            tokens,
            token_index: 0,
            line: self.line,
            op_group_index,
            paren_matches: self.paren_matches,
            types: self.types,
            namespace: self.namespace.clone(),
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
static OPERATORS: [phf::Map<&'static str, fn(expression: &Expression) -> Result<Box<dyn ASTNode>, CompilerError>>; OPERATOR_GROUPS] = [
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
) -> Option<fn(expression: &Expression) -> Result<Box<dyn ASTNode>, CompilerError>> {
    if let Some(&parser) = OPERATORS[expression.op_group_index]
        .get(&expression.tokens[expression.token_index]) {
        return Some(parser)
    }

    None
}

fn binary_operation_parser<'a>(expression: &'a mut Expression) -> Result<Box<dyn ASTNode + 'a>, CompilerError> {
    let op_index = expression.token_index;
    let op_group_index = expression.op_group_index;

    expression.tokens = &expression.tokens[..op_index];
    let mut sub_expr = expression.sub_expression(expression.tokens, op_group_index);
    let left: Box<dyn ASTNode + 'a> = parse_expression(&mut sub_expr)?;

    expression.op_group_index = op_group_index + 1;
    expression.tokens = &expression.tokens[op_group_index+1..];
    let right: Box<dyn ASTNode + 'a> = parse_expression(expression)?;

    if left.get_type() != right.get_type() {
        Err(BinaryOperatorTypeError(
            expression.line.line_num, expression[op_index].to_string(),
            left.get_type().to_string(), right.get_type().to_string())
        )
    } else {
        let x = Box::new(BinaryOperator::new(left, right));
        Ok(x)
    }
}

fn add_parser(expression: &Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Err(InvalidExpression(expression.line.line_num))
}

fn sub_parser(expression: &Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Err(InvalidExpression(expression.line.line_num))
}

fn mul_parser(expression: &Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Err(InvalidExpression(expression.line.line_num))
}

fn div_parser(expression: &Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Err(InvalidExpression(expression.line.line_num))
}

fn mod_parser(expression: &Expression) -> Result<Box<dyn ASTNode>, CompilerError> {
    Err(InvalidExpression(expression.line.line_num))
}

fn assignment_parser<'a>(expression: &'a mut Expression) -> Result<Box<dyn ASTNode + 'a>, CompilerError> {
    if expression.token_index != 1 {
        return Err(InvalidAssignment(expression.line.line_num));
    }

    let var_node_opt = expression.namespace.get_var(&expression.tokens[0]);
    if var_node_opt.is_none() {
        return Err(InvalidSymbol(expression.line.line_num, expression.tokens[0].to_string()));
    }

    let var_node = Box::new(var_node_opt.unwrap().clone());
    let mut assignment_expr = expression.sub_expression(
        &expression.tokens[expression.token_index + 1..], 0);

    Ok(Box::new(BinaryOperator::new(var_node, parse_expression(&mut assignment_expr)?)))
}

fn parse_parenthetical_expression<'a>(expression: &'a mut Expression) -> Result<Box<dyn ASTNode +'a>, CompilerError> {
    expression.tokens = &expression.tokens[1..expression.tokens.len() - 1];
    expression.op_group_index = 0;
    parse_expression(expression)
}

fn parse_value<'a>(expression: &'a Expression<'a>) -> Result<Box<dyn ASTNode + 'a>, CompilerError> {
    let token = &expression.tokens[expression.token_index];

    if let Some(literal_type) = get_literal_type(expression.types, token) {
        return Ok(Box::new(Literal::new(literal_type.clone(), token)));
    }
    if let Some(var_node) = expression.namespace.get_var(token) {
        return Ok(Box::new(var_node));
    }

    Err(InvalidSymbol(expression.line.line_num, token.to_string()))
}

fn parse_expression<'a>(expression: &'a mut Expression<'a>) -> Result<Box<dyn ASTNode + 'a>, CompilerError> {
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
                    // incorrect lifetime
                    return operator(&expression);
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
    Err(InvalidExpression(expression.line.line_num))
}

fn match_parens(expression: &Expression) -> Vec<usize> {
    let paren_matches = vec![0; expression.len()];
    let mut open_parens = Vec::new();

    for (i, token) in expression.tokens.iter().enumerate() {
        if token == PAREN_OPEN {
            open_parens.push(i);
        } else if token == PAREN_CLOSE {
            if open_parens.len() == 0 {
                raise_compiler_error(MismatchedParentheses(expression.line.line_num));
            }
            paren_matches[i] = open_parens.pop().unwrap();
        }
    }

    if open_parens.len() != 0 {
        raise_compiler_error(MismatchedParentheses(expression.line.line_num));
    }

    paren_matches
}

fn expression_node(line: &Line,
                   start: usize,
                   end: usize,
                   types: &HashMap<String, Rc<Type>>,
                   namespace: Rc<Namespace>
) -> Result<Box<dyn ASTNode>, CompilerError> {
    let mut expression = Expression::new(line, start, end, types, namespace);
    expression.paren_matches = &match_parens(&expression);
    parse_expression(&mut expression)
}
