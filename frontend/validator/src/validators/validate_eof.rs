use crate::constants::ERR__EMPTY_EXPR;
use crate::Validator;
use ast::token::TokenKind;
use error::FrontendError;

impl Validator {
    pub(crate) fn validate_eof(&self) -> Result<(), FrontendError> {
        match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Eof => (),
                _ => return Ok(()),
            },
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
