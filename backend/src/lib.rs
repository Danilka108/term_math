mod constants;

use ast::node::{AstNode, BinOpKind, BinOpNode, NumNode, UnOpKind, UnOpNode};
use ast::span::Span;
use constants::{
    ERR__UNDEFINED_LEFT_OPERAND, ERR__UNDEFINED_OPERAND, ERR__UNDEFINED_RIGHT_OPERAND,
    ERR__UNKNOWN_ERROR_UNDEFINED_RESULT, WARN__NAN_VALUE,
};
use notification::Notification;
use number::Number;

pub struct Backend {
    expr: String,
    node_stack: Vec<Box<AstNode>>,
    number_stack: Vec<Number>,
    curr_node: Option<Box<AstNode>>,
    last_visited_node: Option<Box<AstNode>>,
    notifications: Vec<Notification>,
}

impl Backend {
    pub fn new(expr: &str, ast_node: Box<AstNode>) -> Self {
        Self {
            expr: expr.to_string(),
            node_stack: Vec::new(),
            number_stack: Vec::new(),
            curr_node: Some(ast_node),
            last_visited_node: None,
            notifications: Vec::new(),
        }
    }

    fn push_error(&mut self, msg: &str, span: Span) {
        self.notifications.push(Notification::new_error(
            &self.expr,
            msg.to_string(),
            span.start(),
            span.end(),
        ));
    }

    fn push_warn(&mut self, msg: &str, span: Span) {
        self.notifications.push(Notification::new_warning(
            &self.expr,
            msg.to_string(),
            span.start(),
            span.end(),
        ));
    }

    fn visit_bin_op(&mut self, bin_op_node: BinOpNode) {
        let right_operand = match self.number_stack.pop() {
            Some(number) => number,
            None => {
                self.push_error(ERR__UNDEFINED_RIGHT_OPERAND, bin_op_node.span());
                Number::get_nan()
            }
        };
        let left_operand = match self.number_stack.pop() {
            Some(number) => number,
            None => {
                self.push_error(ERR__UNDEFINED_LEFT_OPERAND, bin_op_node.span());
                Number::get_nan()
            }
        };

        let number = match bin_op_node.kind() {
            BinOpKind::Add => left_operand + right_operand,
            BinOpKind::Sub => left_operand - right_operand,
            BinOpKind::Mul => left_operand * right_operand,
            BinOpKind::Div => left_operand / right_operand,
        };

        if number.is_nan() {
            self.push_warn(WARN__NAN_VALUE, bin_op_node.span());
        }

        self.number_stack.push(number);
    }

    fn visit_un_op(&mut self, un_op_node: UnOpNode) {
        let operand = match self.number_stack.pop() {
            Some(number) => number,
            None => {
                self.push_warn(ERR__UNDEFINED_OPERAND, un_op_node.span());
                Number::get_nan()
            }
        };

        let number = match un_op_node.kind() {
            UnOpKind::Neg => -operand,
        };

        if number.is_nan() {
            self.push_warn(WARN__NAN_VALUE, un_op_node.span());
        }

        self.number_stack.push(number);
    }

    fn visit_num(&mut self, num_node: NumNode) {
        let number = Number::from_number_node(num_node.clone());

        if number.is_nan() {
            self.push_warn(WARN__NAN_VALUE, num_node.span());
        }

        self.number_stack.push(number);
    }

    pub fn traverse_ast(mut self) -> (Number, Vec<Notification>) {
        loop {
            if let Some(curr_node) = self.curr_node {
                self.node_stack.push(curr_node.clone());

                self.curr_node = match *curr_node {
                    AstNode::BinOp(bin_op_node) => Some(bin_op_node.left_operand()),
                    AstNode::UnOp(un_op_node) => Some(un_op_node.operand()),
                    AstNode::Num(_) => None,
                    AstNode::FnCall(_) => todo!(),
                };

                continue;
            }

            let parent_node = match self.node_stack.pop() {
                Some(node) => match *node.clone() {
                    AstNode::BinOp(op_node)
                        if Some(op_node.right_operand()) != self.last_visited_node =>
                    {
                        self.node_stack.push(node);
                        self.curr_node = Some(op_node.right_operand());
                        continue;
                    }
                    node => node,
                },
                None => break,
            };

            match parent_node.clone() {
                AstNode::BinOp(bin_op_node) => {
                    self.visit_bin_op(bin_op_node);
                }
                AstNode::UnOp(un_op_node) => {
                    self.visit_un_op(un_op_node);
                }
                AstNode::Num(num_node) => {
                    self.visit_num(num_node);
                }
                AstNode::FnCall(_) => todo!(),
            }

            self.last_visited_node = Some(Box::new(parent_node));
        }

        let result_number = match self.number_stack.pop() {
            None => {
                self.push_error(
                    ERR__UNKNOWN_ERROR_UNDEFINED_RESULT,
                    Span::new(0, self.expr.len()),
                );
                Number::get_nan()
            }
            Some(number) => number,
        };

        (result_number, self.notifications)
    }
}
