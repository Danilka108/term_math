use crate::parser::{BufferElement, Parser};
use ast::node::{AstNode, FnCallNode};
use notification::Notification;
use token::{LiteralKind, TokenKind};

impl Parser {
    fn parse_ident(&mut self) -> Result<(), Notification> {
        let ident_val = match self.get_curr_token_kind() {
            Some(TokenKind::Ident(val)) => val,
            _ => return Ok(()),
        };

        let span = match self.token_stream.curr() {
            Some(token) => token.span(),
            _ => return Ok(()),
        };

        let fn_call_node = Box::new(FnCallNode::new(ident_val, span));
        let buffer_element = BufferElement::FnCall(fn_call_node, false);

        self.buffer.push(buffer_element);

        Ok(())
    }

    fn parse_comma(&mut self) -> Result<(), Notification> {
        match self.get_curr_token_kind() {
            Some(TokenKind::Literal(LiteralKind::Comma)) => (),
            _ => return Ok(()),
        }

        self.parse_ops(|buffer_node| match buffer_node {
            BufferElement::Delim(_) => true,
            _ => false,
        })?;

        let node_pos = if self.buffer.len() >= 2 {
            self.buffer.len() - 2
        } else {
            return Ok(());
        };

        let (fn_call_node, has_args) = match self.buffer.get_mut(node_pos) {
            Some(BufferElement::FnCall(node, has_args)) => (node, has_args),
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

    fn parse_close_paren(&mut self) -> Result<(), Notification> {
        match self.get_curr_token_kind() {
            Some(TokenKind::CloseDelim(_)) => (),
            _ => return Ok(()),
        }

        let (mut fn_call_node, has_args) = match self.buffer.pop() {
            Some(BufferElement::FnCall(node, has_args)) => (node, has_args),
            Some(buffer_node) => {
                self.buffer.push(buffer_node);
                return Ok(());
            }
            _ => return Ok(()),
        };

        match self.output.pop() {
            Some(node) if has_args => fn_call_node.push_arg(node),
            Some(node) if !has_args => self.output.push(node),
            _ => return Err(self.get_unknown_err()),
        }

        let node = AstNode::FnCall(*fn_call_node);
        self.output.push(Box::new(node));

        Ok(())
    }

    pub(crate) fn parse_fn_call(&mut self) -> Result<(), Notification> {
        self.parse_ident()?;
        self.parse_comma()?;
        self.parse_close_paren()
    }
}
