use crate::parser::{BufferNode, Parser};
use token::FromToken;
use ast::{AstNode, OperatorNode};
use error::FrontendError;

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

    fn build_operator(&mut self, operator: OperatorNode) -> Result<(), FrontendError> {
        let (left_operand, right_operand) = self.get_operands()?;

        let node = AstNode::Operator(
            operator
                .set_left_operand(left_operand)
                .set_right_operand(right_operand),
        );

        self.output.push(node);

        Ok(())
    }

    pub(crate) fn parse_operator(&mut self) -> Result<(), FrontendError> {
        let new_operator = match self.token_stream.curr() {
            Some(token) => match OperatorNode::from_token(token) {
                Some(op) => op,
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        match self.buffer.pop() {
            Some(BufferNode::Operator(last_operator))
                if last_operator.priority() <= new_operator.priority() =>
            {
                self.build_operator(last_operator)?
            }
            Some(node) => self.buffer.push(node),
            _ => (),
        }

        self.buffer.push(BufferNode::Operator(new_operator));

        Ok(())
    }

    pub(crate) fn parse_operators(
        &mut self,
        mut predicate: impl FnMut(Option<&BufferNode>) -> bool,
    ) -> Result<(), FrontendError> {
        while let Some(operator) = match self.buffer.pop() {
            Some(BufferNode::Operator(operator)) => Some(operator),
            Some(buffer_node) if predicate(Some(&buffer_node)) => {
                self.buffer.push(buffer_node);
                None
            },
            None if predicate(None) => None,
            _ => return Err(self.get_unknown_err()),
        } {
            self.build_operator(operator)?;
        }

        Ok(())
    }
}
