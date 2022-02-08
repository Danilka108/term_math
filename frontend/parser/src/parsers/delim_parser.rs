use crate::parser::{BufferNode, Parser};
use ast::token::TokenKind;
use error::FrontendError;

impl Parser {
    pub fn parse_open_delim(&mut self) -> Result<(), FrontendError> {
        let delim_kind = match self.get_curr_token_kind() {
            Some(TokenKind::OpenDelim(delim)) => delim,
            _ => return Ok(()),
        };

        self.buffer.push(BufferNode::Delim(delim_kind));

        Ok(())
    }

    pub fn parse_close_delim(&mut self) -> Result<(), FrontendError> {
        let delim_kind = match self.get_curr_token_kind() {
            Some(TokenKind::CloseDelim(delim)) => delim,
            _ => return Ok(()),
        };

        self.parse_operators(|buffer_node| match buffer_node {
            Some(BufferNode::Delim(other_delim_kind)) if other_delim_kind.is_eq(&delim_kind) => {
                true
            }
            _ => false,
        })?;

        match self.buffer.pop() {
            Some(BufferNode::Delim(_)) => Ok(()),
            _ => Err(self.get_unknown_err()),
        }
    }

    pub(crate) fn parse_delim(&mut self) -> Result<(), FrontendError> {
        self.parse_open_delim()?;
        self.parse_close_delim()
    }
}
