use crate::constants::{ERR__INVALID_LEFT_OPERAND, ERR__INVALID_RIGHT_OPERAND};
use crate::Validator;
use ast::token::{LiteralToken, Token, TokenKind};
use error::FrontendError;

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

    fn validate_binary_operator(&self) -> Result<(), FrontendError> {
        let curr_token = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Literal(lit) => match lit {
                    LiteralToken::Asterisk
                    | LiteralToken::Plus
                    | LiteralToken::Slash
                    | LiteralToken::Hyphen => token,
                    _ => return Ok(()),
                },
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        if !self.is_left_operand_valid(self.token_stream.prev()) {
            return Err(FrontendError::from_token(
                self.token_stream.expr(),
                ERR__INVALID_LEFT_OPERAND,
                curr_token,
            ));
        }

        if !self.is_right_operand_valid(self.token_stream.next()) {
            return Err(FrontendError::from_token(
                self.token_stream.expr(),
                ERR__INVALID_RIGHT_OPERAND,
                curr_token,
            ));
        }

        Ok(())
    }

    fn validate_unary_operator(&self) -> Result<(), FrontendError> {
        let is_left_operand_valid = |operand_token: Option<&Token>| -> bool {
            match operand_token {
                Some(token) => match token.kind() {
                    TokenKind::OpenDelim(_)
                    | TokenKind::CloseDelim(_)
                    | TokenKind::Literal(LiteralToken::Comma)
                    | TokenKind::Number(_) => true,
                    _ => false,
                },
                _ => true,
            }
        };

        let curr_token = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Literal(lit) => match lit {
                    LiteralToken::Plus | LiteralToken::Hyphen => token,
                    _ => return Ok(()),
                },
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        if !is_left_operand_valid(self.token_stream.prev()) {
            return Err(FrontendError::from_token(
                self.token_stream.expr(),
                ERR__INVALID_LEFT_OPERAND,
                curr_token,
            ));
        }

        if !self.is_right_operand_valid(self.token_stream.next()) {
            return Err(FrontendError::from_token(
                self.token_stream.expr(),
                ERR__INVALID_RIGHT_OPERAND,
                curr_token,
            ));
        }

        Ok(())
    }

    pub(crate) fn validate_operator(&self) -> Result<(), FrontendError> {
        self.validate_unary_operator()?;
        self.validate_binary_operator()
    }
}