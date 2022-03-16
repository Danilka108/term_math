use crate::constants::{ERR__INVALID_LEFT_OPERAND, ERR__INVALID_RIGHT_OPERAND};
use crate::Validator;
use token::{LiteralKind, Token, TokenKind};
use notification::Notification;
use crate::ErrorFromToken;

impl Validator {
    fn is_left_operand_valid(&self, operand_token: Option<&Token>) -> bool {
        match operand_token {
            Some(token) => match token.kind() {
                TokenKind::CloseDelim(_) | TokenKind::Number(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn is_right_operand_valid(&self, operand_token: Option<&Token>) -> bool {
        match operand_token {
            Some(token) => match token.kind() {
                TokenKind::OpenDelim(_) | TokenKind::Number(_) | TokenKind::Ident(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn validate_binary_op(&self) -> Result<(), Notification> {
        let curr_token = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Literal(lit) => match lit {
                    LiteralKind::Asterisk
                    | LiteralKind::Plus
                    | LiteralKind::Slash
                    | LiteralKind::Hyphen => token,
                    _ => return Ok(()),
                },
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        if !self.is_left_operand_valid(self.token_stream.prev()) {
            return Err(Notification::new_error_from_token(
                self.token_stream.expr(),
                ERR__INVALID_LEFT_OPERAND,
                curr_token,
            ));
        }

        if !self.is_right_operand_valid(self.token_stream.next()) {
            return Err(Notification::new_error_from_token(
                self.token_stream.expr(),
                ERR__INVALID_RIGHT_OPERAND,
                curr_token,
            ));
        }

        Ok(())
    }

    fn validate_unary_op(&self) -> Result<(), Notification> {
        let is_left_operand_valid = |operand_token: Option<&Token>| -> bool {
            match operand_token {
                Some(token) => match token.kind() {
                    TokenKind::OpenDelim(_)
                    | TokenKind::CloseDelim(_)
                    | TokenKind::Literal(LiteralKind::Comma)
                     => true,
                    _ => false,
                },
                _ => true,
            }
        };

        let curr_token = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Literal(lit) => match lit {
                    LiteralKind::Hyphen => token,
                    _ => return Ok(()),
                },
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        if !is_left_operand_valid(self.token_stream.prev()) {
            return Err(Notification::new_error_from_token(
                self.token_stream.expr(),
                ERR__INVALID_LEFT_OPERAND,
                curr_token,
            ));
        }

        if !self.is_right_operand_valid(self.token_stream.next()) {
            return Err(Notification::new_error_from_token(
                self.token_stream.expr(),
                ERR__INVALID_RIGHT_OPERAND,
                curr_token,
            ));
        }

        Ok(())
    }

    pub(crate) fn validate_op(&self) -> Result<(), Notification> {
        match (self.validate_unary_op(), self.validate_binary_op()) {
            (Err(_), Err(err)) => Err(err),
            _ => Ok(()),
        }
    }
}