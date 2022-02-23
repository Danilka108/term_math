use crate::constants::ERR__UNKNOWN_ERROR;
use ast::node::{AstNode, OpNode, FnCallNode};
use ast::span::Span;
use token::{DelimToken, TokenKind};
use error::Error;
use lexer::TokenStream;
use std::panic::Location;

#[derive(Clone, Debug)]
pub(crate) enum BufferNode {
    Delim(DelimToken),
    Op((OpNode, Span)),
    FnCall((FnCallNode, Span, bool)),
}

pub struct Parser {
    pub(crate) token_stream: TokenStream,
    pub(crate) output: Vec<AstNode>,
    pub(crate) buffer: Vec<BufferNode>,
}

impl Parser {
    const PARSERS: [fn(&mut Self) -> Result<(), Error>; 5] = [
        Self::parse_number,
        Self::parse_op,
        Self::parse_delim,
        Self::parse_fn_call,
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
    pub(crate) fn get_unknown_err(&self) -> Error {
        dbg!(Location::caller());
        Error::new(
            &self.token_stream.expr(),
            ERR__UNKNOWN_ERROR.to_string(),
            0,
            self.token_stream.expr().len(),
        )
    }

    pub(crate) fn get_prev_token_kind(&self) -> Option<TokenKind> {
        Some(self.token_stream.prev()?.kind())
    }

    pub(crate) fn get_curr_token_kind(&self) -> Option<TokenKind> {
        Some(self.token_stream.curr()?.kind())
    }

    pub fn parse(mut self) -> Result<AstNode, Error> {
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
