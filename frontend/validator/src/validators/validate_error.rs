use crate::Validator;
use ast::token::TokenKind;
use error::FrontendError;

impl Validator {
    pub(crate) fn validate_error(&self) -> Result<(), FrontendError> {
        let (token, error_msg) = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Error(error_msg) => (token, error_msg),
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        Err(FrontendError::from_token(
            self.token_stream.expr(),
            error_msg.as_str(),
            token,
        ))
    }
}
