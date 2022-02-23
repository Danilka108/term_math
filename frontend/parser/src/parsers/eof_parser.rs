use crate::parser::Parser;
use token::TokenKind;
use error::Error;

impl Parser {
    pub(crate) fn parse_eof(&mut self) -> Result<(), Error> {
        match self.get_curr_token_kind() {
            Some(TokenKind::Eof) => (),
            _ => return Ok(()),
        }

        self.parse_ops(|buffer_node| match buffer_node {
            None => true,
            _ => false,
        })?;

        if !self.buffer.is_empty() {
            return Err(self.get_unknown_err());
        }

        Ok(())
    }
}
