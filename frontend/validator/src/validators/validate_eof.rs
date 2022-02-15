use crate::constants::ERR__EMPTY_EXPR;
use crate::Validator;
use token::TokenKind;
use error::FrontendError;

impl Validator {
    pub(crate) fn validate_eof(&self) -> Result<(), FrontendError> {
        match self.get_curr_token_kind() {
            Some(TokenKind::Eof) => (),
            _ => return Ok(()),
        }

        match self.token_stream.prev() {
            None => Err(FrontendError::new(
                self.token_stream.expr(),
                ERR__EMPTY_EXPR.to_string(),
                0,
                0,
            )),
            _ => Ok(()),
        }
    }
}
