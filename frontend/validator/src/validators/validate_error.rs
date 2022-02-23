use crate::Validator;
use token::TokenKind;
use error::Error;
use crate::FromToken;

impl Validator {
    pub(crate) fn validate_error(&self) -> Result<(), Error> {
        let (token, error_msg) = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Error(error_msg) => (token, error_msg),
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        Err(Error::from_token(
            self.token_stream.expr(),
            error_msg.as_str(),
            token,
        ))
    }
}
