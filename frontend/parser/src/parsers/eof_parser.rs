use crate::parser::Parser;
use token::TokenKind;
use notification::Notification;

impl Parser {
    pub(crate) fn parse_eof(&mut self) -> Result<(), Notification> {
        match self.get_curr_token_kind() {
            Some(TokenKind::Eof) => (),
            _ => return Ok(()),
        }

        self.parse_ops(|_| false)?;

        if !self.buffer.is_empty() {
            return Err(self.get_unknown_err());
        }

        Ok(())
    }
}
