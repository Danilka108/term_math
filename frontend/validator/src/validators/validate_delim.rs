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
        let validate_to_emptiness = |token: &Token| match self.delim_stack.last() {
            Some(element) if element.kind.is_eq(&curr_delim_kind) && element.is_present_ident => {
                Ok(())
            }
            _ => {
                let start = token.span().start();
                let end = curr_token.span().end();

                Err(FrontendError::new(
                    self.token_stream.expr(),
                    ERR__EMPTY_DELIM_BLOCK.to_string(),
                    start,
                    end,
                ))
            }
        };

        match self.token_stream.prev() {
            Some(token) => match token.kind() {
                TokenKind::OpenDelim(_) => validate_to_emptiness(token)?,
                _ => (),
            },
            _ => (),
        }

        Ok(())
    }

    fn validate_to_presence_open_delim(
        &mut self,
        curr_token: &Token,
        curr_delim_kind: &DelimToken,
    ) -> Result<(), FrontendError> {
        match self.delim_stack.last() {
            Some(element) if element.kind.is_eq(&curr_delim_kind) => {
                self.delim_stack.pop();
                Ok(())
            }
            _ => Err(FrontendError::from_token(
                self.token_stream.expr(),
                ERR__MISSING_OPEN_DELIM,
                curr_token,
            )),
        }
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
        match self.token_stream.next() {
            Some(token) => match token.kind() {
                TokenKind::Literal(
                    LiteralToken::Asterisk
                    | LiteralToken::Plus
                    | LiteralToken::Slash
                    | LiteralToken::Hyphen,
                )
                | TokenKind::Eof
                | TokenKind::CloseDelim(_) => (),
                _ => {
                    return Err(FrontendError::from_token(
                        self.token_stream.expr(),
                        ERR__MISSING_OPERATOR_TO_THE_RIGHT_OF_DELIM,
                        curr_token,
                    ))
                }
            },
            _ => (),
        }

        Ok(())
    }

    fn validate_prev_token(&self, curr_token: &Token) -> Result<(), FrontendError> {
        match self.token_stream.prev() {
            Some(token) => match token.kind() {
                TokenKind::Literal(
                    LiteralToken::Asterisk
                    | LiteralToken::Plus
                    | LiteralToken::Slash
                    | LiteralToken::Hyphen,
                )
                | TokenKind::Ident(_) => Ok(()),
                _ => {
                    return Err(FrontendError::from_token(
                        self.token_stream.expr(),
                        ERR__MISSING_OPERATOR_TO_THE_LEFT_OF_DELIM,
                        curr_token,
                    ))
                }
            },
            _ => Ok(()),
        }
    }

    fn is_present_ident(&self) -> bool {
        match self.token_stream.prev() {
            Some(token) => match token.kind() {
                TokenKind::Ident(_) => true,
                _ => false,
            },
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
        match self.token_stream.curr() {
            Some(token) => match token.kind() {
                TokenKind::Eof => (),
                _ => return Ok(()),
            },
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
