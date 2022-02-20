use crate::parser::{BufferNode, Parser};
use ast::node::{AstNode, FunctionCallNode, NodeKind};
use error::FrontendError;
use token::{LiteralToken, TokenKind};

impl Parser {
    fn parse_ident(&mut self) -> Result<(), FrontendError> {
        let ident_val = match self.get_curr_token_kind() {
            Some(TokenKind::Ident(val)) => val,
            _ => return Ok(()),
        };

        let span = match self.token_stream.curr() {
            Some(token) => token.span(),
            _ => return Ok(()),
        };

        self.buffer.push(BufferNode::FunctionCall((
            FunctionCallNode::new(ident_val),
            span,
            false,
        )));

        Ok(())
    }

    fn parse_comma(&mut self) -> Result<(), FrontendError> {
        match self.get_curr_token_kind() {
            Some(TokenKind::Literal(LiteralToken::Comma)) => (),
            _ => return Ok(()),
        }

        self.parse_operators(|buffer_node| match buffer_node {
            Some(BufferNode::Delim(_)) => true,
            _ => false,
        })?;

        let node_pos = if self.buffer.len() >= 2 {
            self.buffer.len() - 2
        } else {
            return Ok(());
        };

        let (fn_call_node, _, has_args) = match self.buffer.get_mut(node_pos) {
            Some(BufferNode::FunctionCall(node)) => node,
            _ => return Ok(()),
        };

        *has_args = true;

        match self.output.pop() {
            Some(node) => {
                fn_call_node.push_arg(node);
                Ok(())
            }
            _ => Err(self.get_unknown_err()),
        }
    }

    fn parse_close_paren(&mut self) -> Result<(), FrontendError> {
        match self.get_curr_token_kind() {
            Some(TokenKind::CloseDelim(_)) => (),
            _ => return Ok(()),
        }

        let close_paren_span = match self.token_stream.curr() {
            Some(token) => token.span(),
            _ => return Ok(()),
        };

        let (mut fn_call_node, ident_span, has_args) = match self.buffer.pop() {
            Some(BufferNode::FunctionCall(node)) => node,
            Some(buffer_node) => {
                self.buffer.push(buffer_node);
                return Ok(());
            }
            _ => return Ok(()),
        };

        if has_args {
            match self.output.pop() {
                Some(node) => fn_call_node.push_arg(node),
                _ => return Err(self.get_unknown_err()),
            }
        }

        self.output.push(AstNode::new(
            NodeKind::FunctionCall(fn_call_node),
            ident_span.concat(&close_paren_span),
        ));

        Ok(())
    }

    pub(crate) fn parse_function_call(&mut self) -> Result<(), FrontendError> {
        self.parse_ident()?;
        self.parse_comma()?;
        self.parse_close_paren()
    }
}
