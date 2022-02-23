use crate::parser::{BufferNode, Parser};
use ast::node::{AstNode, NodeKind, OpNode, BinOpNode, UnOpNode};
use ast::span::Span;
use error::Error;
use token::{FromToken, TokenKind, LiteralToken};

impl Parser {
    fn get_operands_for_bin_op(&mut self) -> Result<(AstNode, AstNode), Error> {
        let right_operand = if let Some(node) = self.output.pop() {
            node
        } else {
            return Err(self.get_unknown_err());
        };

        let left_operand = if let Some(node) = self.output.pop() {
            node
        } else {
            return Err(self.get_unknown_err());
        };

        Ok((left_operand, right_operand))
    }

    fn get_un_operands_for_un_op(&mut self) -> Result<AstNode, Error> {
        if let Some(node) = self.output.pop() {
            Ok(node)
        } else {
            Err(self.get_unknown_err())
        }
    }

    fn build_bin_op(&mut self, span: Span, operator: BinOpNode) -> Result<(), Error> {
        let (left_operand, right_operand) = self.get_operands_for_bin_op()?;

        let kind = NodeKind::Op(OpNode::Bin(operator.set_left_operand(left_operand).set_right_operand(right_operand)));
        let node = AstNode::new(kind, span);
        self.output.push(node);

        Ok(())
    }

    fn build_un_op(&mut self, span: Span, operator: UnOpNode) -> Result<(), Error> {
        let right_operand = self.get_un_operands_for_un_op()?;

        let kind = NodeKind::Op(OpNode::Un(operator.set_right_operand(right_operand)));
        let node = AstNode::new(kind, span);
        self.output.push(node);

        Ok(())
    }

    fn build_op(&mut self, span: Span, operator: OpNode) -> Result<(), Error> {
        match operator {
            OpNode::Bin(bin_op) => self.build_bin_op(span, bin_op),
            OpNode::Un(un_op) => self.build_un_op(span, un_op),
        }        
    }

    pub(crate) fn parse_op(&mut self) -> Result<(), Error> {
        let is_unary = match self.get_prev_token_kind() {
            Some(kind) => match kind {
                TokenKind::Literal(LiteralToken::Plus | LiteralToken::Slash | LiteralToken::Hyphen | LiteralToken::Asterisk) | TokenKind::OpenDelim(_) => false,
                _ => true,
            }
            None => true,
        };

        let (new_operator, span) = match self.token_stream.curr() {
            Some(token) => match OpNode::from_token(token, is_unary) {
                Some(op) => (op, token.span()),
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        match self.buffer.pop() {
            Some(BufferNode::Op((last_operator, last_operator_span)))
                if last_operator.priority() <= new_operator.priority() =>
            {
                self.build_op(last_operator_span, last_operator)?
            }
            Some(node) => self.buffer.push(node),
            _ => (),
        }

        self.buffer.push(BufferNode::Op((new_operator, span)));

        Ok(())
    }

    pub(crate) fn parse_ops(
        &mut self,
        mut predicate: impl FnMut(Option<&BufferNode>) -> bool,
    ) -> Result<(), Error> {
        while let Some((operator, span)) = match self.buffer.pop() {
            Some(BufferNode::Op(operator)) => Some(operator),
            Some(buffer_node) if predicate(Some(&buffer_node)) => {
                self.buffer.push(buffer_node);
                None
            }
            None if predicate(None) => None,
            _ => return Err(self.get_unknown_err()),
        } {
            self.build_op(span, operator)?;
        }

        Ok(())
    }
}
