use crate::Validator;
use token::TokenKind;
use notification::Notification;
use crate::ErrorFromToken;

impl Validator {
    pub(crate) fn validate_error(&self) -> Result<(), Notification> {
        let (token, error_msg) = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Error(error_msg) => (token, error_msg),
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        Err(Notification::new_error_from_token(
            self.token_stream.expr(),
            error_msg.as_str(),
            token,
        ))
    }
}
