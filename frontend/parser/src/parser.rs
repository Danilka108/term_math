use crate::constants::ERR__UNKNOWN_ERROR;
use ast::{AstNode, OperatorNode, FunctionCallNode};
use token::{DelimToken, TokenKind};
use error::FrontendError;
use lexer::TokenStream;
use std::panic::Location;

#[derive(Clone, Debug)]
pub(crate) enum BufferNode {
    Delim(DelimToken),
    Operator(OperatorNode),
    FunctionCall((FunctionCallNode, bool)),
}

pub struct Parser {
    pub(crate) token_stream: TokenStream,
    pub(crate) output: Vec<AstNode>,
    pub(crate) buffer: Vec<BufferNode>,
}

impl Parser {
    const PARSERS: [fn(&mut Self) -> Result<(), FrontendError>; 5] = [
        Self::parse_number,
        Self::parse_operator,
        Self::parse_delim,
        Self::parse_function_call,
        Self::parse_eof,
    ];

    pub fn new(token_stream: &TokenStream) -> Self {
        Self {
            token_stream: token_stream.clone(),
            output: Vec::new(),
            buffer: Vec::new(),
        }
    }

    #[track_caller]
    pub(crate) fn get_unknown_err(&self) -> FrontendError {
        dbg!(Location::caller());
        FrontendError::new(
            &self.token_stream.expr(),
            ERR__UNKNOWN_ERROR.to_string(),
            0,
            self.token_stream.expr().len(),
        )
    }

    pub(crate) fn get_curr_token_kind(&self) -> Option<TokenKind> {
        Some(self.token_stream.curr()?.kind())
    }

    pub fn parse(mut self) -> Result<AstNode, FrontendError> {
        while let Some(_) = self.token_stream.to_next() {
            for parse in Self::PARSERS {
                parse(&mut self)?;
            }
        }

        match self.output.pop() {
            Some(node) if self.output.is_empty() => Ok(node),
            _ => Err(self.get_unknown_err()),
        }
    }
}
