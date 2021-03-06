use crate::errors::*;
use ir::ast::*;
use ir::span::*;
use ir::token::*;

use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum BuffElem {
    BinOp(BinOpKind),
    UnOp(UnOpKind),
    Fn(String, usize),
    Delim(DelimKind),
}



#[derive(Clone, Debug)]
pub struct Parser {
    tokens: TokenStream,
    expr_span: Span,
    curr_token: SpanWrapper<Token>,
    nodes: Vec<SpanWrapper<Node>>,
    buffer: Vec<SpanWrapper<BuffElem>>,
}

pub type PResult<O = ()> = Result<O, SpanWrapper<String>>;

impl Parser {
    const PARSERS: [fn(&mut Self) -> Result<(), SpanWrapper<String>>; 4] = [
        Self::parse_fn,
        Self::parse_delimited,
        Self::parse_num,
        Self::parse_op,
    ];

    pub fn new(token_stream: TokenStream) -> Self {
        let expr_len = match token_stream
            .clone()
            .filter(|w| w.borrow_val() != &Token::Eof)
            .last()
        {
            Some(wrapper) => wrapper.span().end(),
            None => 0,
        };

        Self {
            tokens: token_stream,
            expr_span: Span::new(0, expr_len),
            curr_token: SpanWrapper::new(Token::Eof, Span::new(0, 0)),
            nodes: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.tokens.clone().collect::<Vec<_>>().is_empty()
    }

    pub(crate) fn eof(&self) -> SpanWrapper<Token> {
        SpanWrapper::new(Token::Eof, self.expr_span.clone())
    }

    pub(crate) fn curr(&self) -> SpanWrapper<Token> {
        self.curr_token.clone()
    }

    pub(crate) fn first(&self) -> SpanWrapper<Token> {
        self.tokens.clone().next().unwrap_or(self.eof())
    }

    pub(crate) fn second(&self) -> SpanWrapper<Token> {
        let mut cloned = self.tokens.clone();
        cloned.next();
        cloned.next().unwrap_or(self.eof())
    }

    pub(crate) fn bump(&mut self) {
        if let Some(tok) = self.tokens.next() {
            self.curr_token = tok;
        };
    }

    pub(crate) fn pop_node_or(
        &mut self,
        err: &str,
        span: Span,
    ) -> Result<SpanWrapper<Node>, SpanWrapper<String>> {
        match self.nodes.pop() {
            Some(node) => Ok(node),
            None => Err(SpanWrapper::new(err.to_owned(), span)),
        }
    }

    pub(crate) fn push_node(&mut self, node: SpanWrapper<Node>) {
        self.nodes.push(node)
    }

    pub(crate) fn push_buff_tuple(&mut self, buff_elem: BuffElem, span: Span) {
        self.buffer.push(SpanWrapper::new(buff_elem, span))
    }

    pub(crate) fn pop_buff_tuple(&mut self) -> Option<(BuffElem, Span)> {
        self.buffer.pop().map(|w| w.to_tuple())
    }

    pub(crate) fn pop_buff_tuple_or(
        &mut self,
        err: &str,
        span: Span,
    ) -> Result<(BuffElem, Span), SpanWrapper<String>> {
        match self.buffer.pop() {
            Some(v) => Ok(v.to_tuple()),
            None => Err(SpanWrapper::new(err.to_owned(), span)),
        }
    }

    pub(crate) fn mut_last_buff_tuple(&mut self) -> Option<(&mut BuffElem, &mut Span)> {
        self.buffer.last_mut().map(|t| t.mut_borrow_to_tuple())
    }

    pub(crate) fn new_err<R>(msg: &str, span: Span) -> Result<R, SpanWrapper<String>> {
        Err(SpanWrapper::new(msg.to_owned(), span))
    }

    fn parse_unknown(&mut self) -> Result<(), SpanWrapper<String>> {
        if matches!(self.first().val(), Token::Unknown) {
            Err(SpanWrapper::new(
                ERR__UNKNOWN_SYMBOLS.to_owned(),
                self.first().span(),
            ))
        } else {
            Ok(())
        }
    }

    fn filter_whitespaces(&mut self) {
        self.tokens = self
            .tokens
            .clone()
            .into_iter()
            .filter(|wrapper| !matches!(wrapper.borrow_val(), Token::Whitespace))
            .collect::<Vec<_>>()
            .into_iter();
    }

    fn parse_eof(&mut self) -> Result<Box<SpanWrapper<Node>>, SpanWrapper<String>> {
        self.collect_ops()?;

        match self.buffer.pop() {
            Some(elem) if matches!(elem.borrow_val(), BuffElem::Delim(_)) => {
                return Err(SpanWrapper::new(
                    ERR__UNCLOSED_DELIMITED_BLOCK.to_owned(),
                    elem.span(),
                ))
            }
            Some(elem) if matches!(elem.borrow_val(), BuffElem::Fn(_, _)) => {
                return Err(SpanWrapper::new(
                    ERR__MISSING_ARGS_BLOCK_END.to_owned(),
                    elem.span(),
                ))
            }
            Some(_) => {
                return Err(SpanWrapper::new(
                    ERR__PARSING_ERR.to_owned(),
                    self.expr_span.clone(),
                ))
            }
            _ => (),
        }

        let ast = match self.nodes.pop() {
            Some(node) if self.nodes.len() == 0 => Box::new(node),
            _ => {
                return Err(SpanWrapper::new(
                    ERR__PARSING_ERR.to_owned(),
                    self.expr_span.clone(),
                ))
            }
        };

        Ok(ast)
    }

    pub fn build_ast(mut self) -> Result<Box<SpanWrapper<Node>>, SpanWrapper<String>> {
        self.filter_whitespaces();

        if self.is_eof() {
            return Err(SpanWrapper::new(ERR__EMPTY_EXPR.to_owned(), self.expr_span));
        }

        while !self.is_eof() {
            self.parse_unknown()?;

            for parse in Self::PARSERS {
                parse(&mut self)?;
            }
            self.bump();
        }

        self.parse_eof()
    }
}
