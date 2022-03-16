use crate::constants::ERR__NUMBER_IS_NOT_OPERAND_OR_FN_ARG;
use crate::ErrorFromToken;
use crate::Validator;
use notification::Notification;
use token::{LiteralKind, TokenKind};

impl Validator {
    pub(crate) fn validate_num(&self) -> Result<(), Notification> {
        let curr_token = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Number(_) => token,
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        match self.get_curr_token_kind() {
            Some(TokenKind::Number(_)) => (),
            _ => return Ok(()),
        }

        let is_valid_left_token = match self.get_prev_token_kind() {
            Some(TokenKind::Literal(
                LiteralKind::Plus
                | LiteralKind::Slash
                | LiteralKind::Hyphen
                | LiteralKind::Asterisk
                | LiteralKind::Comma,
            )) => true,
            _ => false,
        };

        let is_valid_right_token = match self.get_next_token_kind() {
            Some(TokenKind::Literal(
                LiteralKind::Plus
                | LiteralKind::Slash
                | LiteralKind::Hyphen
                | LiteralKind::Asterisk
                | LiteralKind::Comma,
            )) => true,
            _ => false,
        };

        if is_valid_right_token || is_valid_left_token {
            Ok(())
        } else {
            Err(Notification::new_error_from_token(
                self.token_stream.expr(),
                ERR__NUMBER_IS_NOT_OPERAND_OR_FN_ARG,
                curr_token,
            ))
        }
    }
}