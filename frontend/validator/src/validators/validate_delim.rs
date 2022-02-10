use crate::constants::{
    ERR__EMPTY_DELIM_BLOCK, ERR__MISSING_CLOSE_DELIM, ERR__MISSING_OPEN_DELIM,
    ERR__MISSING_OPERATOR_TO_THE_LEFT_OF_DELIM, ERR__MISSING_OPERATOR_TO_THE_RIGHT_OF_DELIM,
};
use crate::validator::{DelimStackElement, Validator};
use ast::token::{DelimToken, LiteralToken, Token, TokenKind};
use error::FrontendError;

impl Validator {
    fn validate_delim_block_to_emptiness(
        &self,
        curr_token: &Token,
        curr_delim_kind: &DelimToken,
    ) -> Result<(), FrontendError> {
        let prev_token = match self.token_stream.prev() {
            Some(token) => match token.kind() {
                TokenKind::OpenDelim(_) => token,
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        match self.delim_stack.last() {
            Some(element) if element.kind.is_eq(curr_delim_kind) && element.is_present_ident => {
                return Ok(())
            }
            _ => (),
        }

        let start = prev_token.span().start();
        let end = curr_token.span().end();

        Err(FrontendError::new(
            self.token_stream.expr(),
            ERR__EMPTY_DELIM_BLOCK.to_string(),
            start,
            end,
        ))
    }

    fn validate_to_presence_open_delim(
        &mut self,
        curr_token: &Token,
        curr_delim_kind: &DelimToken,
    ) -> Result<(), FrontendError> {
        match self.delim_stack.last() {
            Some(element) if element.kind.is_eq(curr_delim_kind) => {
                self.delim_stack.pop();
                return Ok(());
            }
            _ => (),
        }

        Err(FrontendError::from_token(
            self.token_stream.expr(),
            ERR__MISSING_OPEN_DELIM,
            curr_token,
        ))
    }

    fn validate_to_presence_close_delim(&self) -> Result<(), FrontendError> {
        if let Some(element) = self.delim_stack.last() {
            Err(FrontendError::from_token(
                self.token_stream.expr(),
                ERR__MISSING_CLOSE_DELIM,
                &element.token,
            ))
        } else {
            Ok(())
        }
    }

    fn validate_next_token(&self, curr_token: &Token) -> Result<(), FrontendError> {
        if let Some(
            TokenKind::Eof
            | TokenKind::CloseDelim(_)
            | TokenKind::Literal(
                LiteralToken::Plus
                | LiteralToken::Hyphen
                | LiteralToken::Asterisk
                | LiteralToken::Slash,
            ),
        ) = self.get_next_token_kind()
        {
            return Ok(());
        }

        Err(FrontendError::from_token(
            self.token_stream.expr(),
            ERR__MISSING_OPERATOR_TO_THE_RIGHT_OF_DELIM,
            curr_token,
        ))
    }

    fn validate_prev_token(&self, curr_token: &Token) -> Result<(), FrontendError> {
        if let None | Some(
            TokenKind::Ident(_)
            | TokenKind::OpenDelim(_)
            | TokenKind::Literal(
                LiteralToken::Plus
                | LiteralToken::Hyphen
                | LiteralToken::Asterisk
                | LiteralToken::Slash,
            ),
        ) = self.get_prev_token_kind()
        {
            return Ok(());
        }

        Err(FrontendError::from_token(
            self.token_stream.expr(),
            ERR__MISSING_OPERATOR_TO_THE_LEFT_OF_DELIM,
            curr_token,
        ))
    }

    fn is_present_ident(&self) -> bool {
        match self.get_prev_token_kind() {
            Some(TokenKind::Ident(_)) => true,
            _ => false,
        }
    }

    fn validate_close_delim(&mut self) -> Result<(), FrontendError> {
        let (curr_token, curr_delim_kind) = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::CloseDelim(delim) => (token.clone(), delim),
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        self.validate_delim_block_to_emptiness(&curr_token, &curr_delim_kind)?;
        self.validate_next_token(&curr_token)?;
        self.validate_to_presence_open_delim(&curr_token, &curr_delim_kind)
    }

    fn validate_open_delim(&mut self) -> Result<(), FrontendError> {
        let (curr_token, curr_delim_kind) = match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::OpenDelim(delim) => (token.clone(), delim),
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        self.validate_prev_token(&curr_token)?;

        self.delim_stack.push(DelimStackElement {
            kind: curr_delim_kind,
            token: curr_token,
            is_present_ident: self.is_present_ident(),
            is_present_args: false,
        });

        Ok(())
    }

    fn validate_delim_stack(&self) -> Result<(), FrontendError> {
        match self.get_curr_token_kind() {
            Some(TokenKind::Eof) => (),
            _ => return Ok(()),
        }

        self.validate_to_presence_close_delim()
    }

    pub(crate) fn validate_delim(&mut self) -> Result<(), FrontendError> {
        self.validate_open_delim()?;
        self.validate_close_delim()?;
        self.validate_delim_stack()
    }
}
