use crate::constants::{
    ERR__INVALID_DELIM_TYPE_OF_FUNC_ARGS_BLOCK, ERR__MISSING_FUNC_ARG,
    ERR__MISSING_FUNC_ARGS_BLOCK, ERR__MISSING_FUNC_IDENT,
};
use crate::Validator;
use token::{DelimKind, LiteralKind, Token, TokenKind};
use notification::Notification;
use crate::ErrorFromToken;

impl Validator {
    fn validate_ident(&self) -> Result<(), Notification> {
        let curr_token = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Ident(_) => token,
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        let err = Notification::new_error_from_token(
            self.token_stream.expr(),
            ERR__MISSING_FUNC_ARGS_BLOCK,
            curr_token,
        );

        match self.token_stream.next() {
            Some(token) => match token.kind() {
                TokenKind::OpenDelim(_) => Ok(()),
                _ => return Err(err),
            },
            _ => return Err(err),
        }
    }

    fn validate_prev_arg_to_missing(&self, curr_token: &Token) -> Result<(), Notification> {
        match self.get_prev_token_kind() {
            Some(TokenKind::OpenDelim(_) | TokenKind::Literal(LiteralKind::Comma)) => (),
            _ => return Ok(()),
        }

        Err(Notification::new_error_from_token(
            self.token_stream.expr(),
            ERR__MISSING_FUNC_ARG,
            curr_token,
        ))
    }

    fn validate_next_arg_to_missing(&self, curr_token: &Token) -> Result<(), Notification> {
        match self.get_next_token_kind() {
            Some(TokenKind::CloseDelim(_) | TokenKind::Literal(LiteralKind::Comma)) => (),
            _ => return Ok(()),
        }

        Err(Notification::new_error_from_token(
            self.token_stream.expr(),
            ERR__MISSING_FUNC_ARG,
            curr_token,
        ))
    }

    fn validate_ident_to_missing(&self, curr_token: &Token) -> Result<(), Notification> {
        match self.delim_stack.last() {
            Some(element) if element.is_present_args && !element.is_present_ident => {
                Err(Notification::new_error(
                    self.token_stream.expr(),
                    ERR__MISSING_FUNC_IDENT.to_string(),
                    element.token.span().start(),
                    curr_token.span().end(),
                ))
            }
            _ => Ok(()),
        }
    }

    fn validate_delim_type(&self, curr_token: &Token) -> Result<(), Notification> {
        match self.delim_stack.last() {
            Some(element) if element.is_present_ident && !element.kind.is_eq(&DelimKind::Paren) => Err(Notification::new_error(
                self.token_stream.expr(),
                ERR__INVALID_DELIM_TYPE_OF_FUNC_ARGS_BLOCK.to_string(),
                element.token.span().start(),
                curr_token.span().end(),
            )),
            _ => Ok(()),
        }
    }

    fn validate_comma(&mut self) -> Result<(), Notification> {
        let curr_token = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Literal(LiteralKind::Comma) => token,
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        match self.delim_stack.last_mut() {
            Some(element) => {
                element.is_present_args = true;
            }
            _ => (),
        }

        self.validate_prev_arg_to_missing(&curr_token)?;
        self.validate_next_arg_to_missing(&curr_token)?;

        Ok(())
    }

    fn validate_close_paren(&self) -> Result<(), Notification> {
        let curr_token = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::CloseDelim(_) => token,
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        self.validate_ident_to_missing(&curr_token)?;
        self.validate_delim_type(&curr_token)?;

        Ok(())
    }

    pub(crate) fn validate_fn_call(&mut self) -> Result<(), Notification> {
        self.validate_ident()?;
        self.validate_comma()?;
        self.validate_close_paren()
    }
}
