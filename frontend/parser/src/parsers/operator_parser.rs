use crate::parser::{BufferNode, Parser};
use ast::node::{AstNode, NodeKind, OperatorNode};
use ast::span::Span;
use error::FrontendError;
use token::FromToken;

impl Parser {
    pub(crate) fn get_operands(&mut self) -> Result<(AstNode, AstNode), FrontendError> {
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

    fn build_operator(&mut self, span: Span, operator: OperatorNode) -> Result<(), FrontendError> {
        let (left_operand, right_operand) = self.get_operands()?;

        let node = AstNode::new(
            NodeKind::Operator(
                operator
                    .set_left_operand(left_operand)
                    .set_right_operand(right_operand),
            ),
            span,
        );
        self.output.push(node);

        Ok(())
    }

    pub(crate) fn parse_operator(&mut self) -> Result<(), FrontendError> {
        let (new_operator, span) = match self.token_stream.curr() {
            Some(token) => match OperatorNode::from_token(token) {
                Some(op) => (op, token.span()),
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        match self.buffer.pop() {
            Some(BufferNode::Operator((last_operator, last_operator_span)))
                if last_operator.priority() <= new_operator.priority() =>
            {
                self.build_operator(last_operator_span, last_operator)?
            }
            Some(node) => self.buffer.push(node),
            _ => (),
        }

        self.buffer.push(BufferNode::Operator((new_operator, span)));

        Ok(())
    }

    pub(crate) fn parse_operators(
        &mut self,
        mut predicate: impl FnMut(Option<&BufferNode>) -> bool,
    ) -> Result<(), FrontendError> {
        while let Some((operator, span)) = match self.buffer.pop() {
            Some(BufferNode::Operator(operator)) => Some(operator),
            Some(buffer_node) if predicate(Some(&buffer_node)) => {
                self.buffer.push(buffer_node);
                None
            }
            None if predicate(None) => None,
            _ => return Err(self.get_unknown_err()),
        } {
            self.build_operator(span, operator)?;
        }

        Ok(())
    }
}
