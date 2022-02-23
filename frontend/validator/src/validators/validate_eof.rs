use crate::constants::ERR__EMPTY_EXPR;
use crate::Validator;
use token::TokenKind;
use error::Error;

impl Validator {
    pub(crate) fn validate_eof(&self) -> Result<(), Error> {
        match self.get_curr_token_kind() {
            Some(TokenKind::Eof) => (),
            _ => return Ok(()),
        }

        match self.token_stream.prev() {
            None => Err(Error::new(
                self.token_stream.expr(),
                ERR__EMPTY_EXPR.to_string(),
                0,
                0,
            )),
            _ => Ok(()),
        }
    }
}
