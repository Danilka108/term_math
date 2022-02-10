use ast::token::{DelimToken, Token, TokenKind};
use error::FrontendError;
use lexer::TokenStream;

#[derive(Clone, Debug)]
pub(crate) struct DelimStackElement {
    pub kind: DelimToken,
    pub token: Token,
    pub is_present_ident: bool,
    pub is_present_args: bool,
}

pub struct Validator {
    pub(crate) delim_stack: Vec<DelimStackElement>,
    pub(crate) token_stream: TokenStream,
}

impl Validator {
    pub fn new(token_stream: &TokenStream) -> Self {
        Self {
            token_stream: token_stream.clone(),
            delim_stack: Vec::new(),
        }
    }
    
    pub(crate) fn get_curr_token_kind(&self) -> Option<TokenKind> {
        match self.token_stream.curr() {
            Some(token) => Some(token.kind()),
            _ => None
        }
    }

    pub(crate) fn get_next_token_kind(&self) -> Option<TokenKind> {
        match self.token_stream.next() {
            Some(token) => Some(token.kind()),
            _ => None
        }
    }

    pub(crate) fn get_prev_token_kind(&self) -> Option<TokenKind> {
        match self.token_stream.prev() {
            Some(token) => Some(token.kind()),
            _ => None
        }
    }

    pub fn validate(mut self) -> Result<(), FrontendError> {
        while let Some(_) = self.token_stream.to_next() {
            self.validate_operator()?;
            self.validate_function_call()?;
            self.validate_delim()?;
            self.validate_eof()?;
            self.validate_error()?;
        }

        Ok(())
    }
}
