//mod build_ast;
mod errors;
//mod expr;
//mod delimited;

use errors::*;
use ir::ast::*;
use ir::cursor::prelude::*;
use ir::cursor::TokenCursor;
use ir::span::*;
use ir::token::*;

use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
enum BuffElem {
    BinOp(BinOpKind),
    UnOp(UnOpKind),
    //Fn(String),
    Delim(DelimKind),
}

#[derive(Clone, Debug)]
pub struct Parser {
    tokens: std::vec::IntoIter<SpanWrapper<Token>>,
    expr_span: Span,
    curr_token: Token,
    nodes: Vec<SpanWrapper<Node>>,
    buffer: Vec<SpanWrapper<BuffElem>>,
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        let expr_len = match token_stream.clone().last() {
            Some(wrapper) => wrapper.span().end(),
            None => 0,
        };

        Self {
            tokens: token_stream,
            expr_span: Span::new(0, expr_len),
            curr_token: Token::Eof,
            nodes: Vec::new(),
            buffer: Vec::new(),
        }
    }

    fn is_eof(&self) -> bool {
        matches!(self.curr_token, Token::Eof) && matches!(self.first().val(), Token::Eof)
    }

    fn eof(&self) -> SpanWrapper<Token> {
        SpanWrapper::new(Token::Eof, self.expr_span.clone())
    }

    fn first(&self) -> SpanWrapper<Token> {
        self.tokens.clone().next().unwrap_or(self.eof())
    }

    fn second(&self) -> SpanWrapper<Token> {
        let mut cloned = self.tokens.clone();
        cloned.next();
        cloned.next().unwrap_or(self.eof())
    }

    fn bump(&mut self) {
        if let Some(tok) = self.tokens.next() {
            self.curr_token = tok.val();
        };
    }

    fn build_bin_op(&mut self, kind: BinOpKind, span: Span) -> Result<(), SpanWrapper<String>> {
        let rhs = match self.nodes.pop() {
            Some(node) => Box::new(node),
            None => {
                return Err(SpanWrapper::new(
                    ERR__MISSIGN_RIGHT_OPERAND.to_owned(),
                    span,
                ))
            }
        };

        let lhs = match self.nodes.pop() {
            Some(node) => Box::new(node),
            None => return Err(SpanWrapper::new(ERR__MISSIGN_LEFT_OPERAND.to_owned(), span)),
        };

        let node = SpanWrapper::new(Node::BinOp(kind, lhs, rhs), span);

        self.nodes.push(node);
        Ok(())
    }

    fn build_un_op(&mut self, kind: UnOpKind, span: Span) -> Result<(), SpanWrapper<String>> {
        let operand = match self.nodes.pop() {
            Some(node) => Box::new(node),
            None => {
                return Err(SpanWrapper::new(
                    ERR__INVALID_RIGHT_OPERAND.to_owned(),
                    span,
                ))
            }
        };

        let node = SpanWrapper::new(Node::UnOp(kind, operand), span);
        self.nodes.push(node);

        Ok(())
    }

    fn collect_ops(&mut self) -> Result<(), SpanWrapper<String>> {
        loop {
            let buff_elem = match self.buffer.pop() {
                Some(elem) => elem,
                None => break,
            };

            match buff_elem.to_tuple() {
                (BuffElem::BinOp(op_kind), span) => self.build_bin_op(op_kind, span)?,
                (BuffElem::UnOp(op_kind), span) => self.build_un_op(op_kind, span)?,
                (val, span) => {
                    self.buffer.push(SpanWrapper::new(val, span));
                    break;
                }
            }
        }

        Ok(())
    }

    fn is_empty_curr(&self) -> bool {
        matches!(self.curr_token, Token::Eof)
    }

    fn is_empty_first(&self) -> bool {
        matches!(self.first().val(), Token::Eof)
    }

    fn is_empty_second(&self) -> bool {
        matches!(self.second().val(), Token::Eof)
    }

    fn is_valid_right_operand(&self) -> bool {
        matches!(
            self.second().val(),
            Token::OpenDelim(_) | Token::Num(_) | Token::Ident(_)
        )
    }

    fn is_valid_left_operand(&self) -> bool {
        matches!(self.curr_token, Token::CloseDelim(_) | Token::Num(_))
    }

    fn parse_bin_op(&mut self) -> Result<(), SpanWrapper<String>> {
        let (new_op_val, new_op_span) = self.first().to_tuple();

        let new_op_kind = match new_op_val {
            Token::Lit(LitKind::Plus) => BinOpKind::Add,
            Token::Lit(LitKind::Hyphen) => BinOpKind::Sub,
            Token::Lit(LitKind::Asterisk) => BinOpKind::Mul,
            Token::Lit(LitKind::Slash) => BinOpKind::Div,
            _ => return Ok(()),
        };

        if self.is_empty_first() {
            return Err(SpanWrapper::new(
                ERR__MISSIGN_RIGHT_OPERAND.to_owned(),
                new_op_span,
            ));
        }

        if !self.is_valid_right_operand() {
            return Err(SpanWrapper::new(
                ERR__INVALID_RIGHT_OPERAND.to_owned(),
                new_op_span,
            ));
        }

        if self.is_empty_curr() {
            return Err(SpanWrapper::new(
                ERR__MISSIGN_LEFT_OPERAND.to_owned(),
                new_op_span,
            ));
        }

        if !self.is_valid_left_operand() {
            return Err(SpanWrapper::new(
                ERR__INVALID_LEFT_OPERAND.to_owned(),
                new_op_span,
            ));
        }

        let (last_op_val, last_op_span) = match self.buffer.pop() {
            Some(val) => val.to_tuple(),
            None => {
                self.buffer
                    .push(SpanWrapper::new(BuffElem::BinOp(new_op_kind), new_op_span));
                return Ok(());
            }
        };

        match last_op_val {
            BuffElem::BinOp(last_op_kind) if last_op_kind >= new_op_kind => {
                self.build_bin_op(last_op_kind, last_op_span)?
            }
            BuffElem::UnOp(last_op_kind) => self.build_un_op(last_op_kind, last_op_span)?,
            val => self.buffer.push(SpanWrapper::new(val, last_op_span)),
        }

        self.buffer
            .push(SpanWrapper::new(BuffElem::BinOp(new_op_kind), new_op_span));

        Ok(())
    }

    fn parse_un_op(&mut self) -> Result<(), SpanWrapper<String>> {
        let (op_val, op_span) = self.first().to_tuple();

        let op_kind = match op_val {
            Token::Lit(LitKind::Hyphen) => UnOpKind::Neg,
            _ => return Ok(()),
        };

        if self.is_empty_first() {
            return Err(SpanWrapper::new(
                ERR__MISSIGN_RIGHT_OPERAND.to_owned(),
                op_span,
            ));
        }

        if !self.is_valid_right_operand() {
            return Err(SpanWrapper::new(
                ERR__INVALID_RIGHT_OPERAND.to_owned(),
                op_span,
            ));
        }

        self.buffer.push(SpanWrapper::new(BuffElem::UnOp(op_kind), op_span));
        Ok(())
    }

    fn parse_op(&mut self) -> Result<(), SpanWrapper<String>> {
        self.parse_bin_op().or(self.parse_un_op())
    }

    fn is_valid_num_bounds(&self) -> bool {
        let is_valid_lhs = matches!(
            self.curr_token,
            Token::Lit(
                LitKind::Asterisk
                    | LitKind::Slash
                    | LitKind::Plus
                    | LitKind::Hyphen
                    | LitKind::Comma
            ) | Token::OpenDelim(_)
                | Token::Eof
        );
        let is_valid_rhs = matches!(
            self.second().val(),
            Token::Lit(
                LitKind::Asterisk
                    | LitKind::Slash
                    | LitKind::Plus
                    | LitKind::Hyphen
                    | LitKind::Comma
            ) | Token::CloseDelim(_)
                | Token::Eof
        );

        let is_lhs_separator = matches!(
            self.curr_token,
            Token::OpenDelim(_) | Token::Eof
        );
        let is_rhs_separator = matches!(
            self.second().val(),
            Token::CloseDelim(_) | Token::Eof
        );

        if is_lhs_separator && is_rhs_separator {
            return false;
        }

        is_valid_rhs || is_valid_lhs
    }

    fn parse_num(&mut self) -> Result<(), SpanWrapper<String>> {
        let (token, span) = self.first().to_tuple();

        let val = match token {
            Token::Num(val) => val,
            _ => return Ok(()),
        };

        if !self.is_valid_num_bounds() {
            return Err(SpanWrapper::new(ERR__MISSIGN_OPERATOR.to_owned(), span));
        }

        let node = SpanWrapper::new(Node::Num(val), span);

        self.nodes.push(node);

        Ok(())
    }

    fn parse_delimited_start(&mut self) -> Result<(), SpanWrapper<String>> {
        let (delim_val, delim_span) = self.first().to_tuple();

        let delim_kind = match delim_val {
            Token::OpenDelim(delim_kind) => delim_kind,
            _ => return Ok(()),
        };

        match self.second().to_tuple() {
            (Token::CloseDelim(close_delim_kind), span) if close_delim_kind == delim_kind => {
                return Err(SpanWrapper::new(
                    ERR__EMPTY_DELIMITED_BLOCK.to_owned(),
                    span,
                ))
            }
            _ => (),
        }

        self.buffer
            .push(SpanWrapper::new(BuffElem::Delim(delim_kind), delim_span));

        Ok(())
    }

    fn parse_delimited_end(&mut self) -> Result<(), SpanWrapper<String>> {
        let (delim_val, delim_span) = self.first().to_tuple();

        let delim_kind = match delim_val {
            Token::CloseDelim(delim_kind) => delim_kind,
            _ => return Ok(()),
        };

        self.collect_ops()?;

        let buff_elem = match self.buffer.pop() {
            Some(elem) => elem,
            None => {
                return Err(SpanWrapper::new(
                    ERR__UNOPENED_DELIMITED_BLOCK.to_owned(),
                    delim_span,
                ))
            }
        };

        match buff_elem.val() {
            BuffElem::Delim(required_delim_kind) if delim_kind == required_delim_kind => Ok(()),
            _ => Err(SpanWrapper::new(
                ERR__UNOPENED_DELIMITED_BLOCK.to_owned(),
                delim_span,
            )),
        }
    }

    fn parse_delimited(&mut self) -> Result<(), SpanWrapper<String>> {
        self.parse_delimited_start()?;
        self.parse_delimited_end()
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

    const PARSERS: [fn(&mut Self) -> Result<(), SpanWrapper<String>>; 4] = [
        Self::parse_unknown,
        Self::parse_delimited,
        Self::parse_num,
        Self::parse_op,
    ];

    pub fn build_ast(mut self) -> Result<Box<SpanWrapper<Node>>, SpanWrapper<String>> {
        self.filter_whitespaces();

        if self.expr_span.len() == 0 {
            return Err(SpanWrapper::new(ERR__EMPTY_EXPR.to_owned(), self.expr_span));
        }

        while !self.is_eof() {
            dbg!(self.first());
            for parse in Self::PARSERS {
                parse(&mut self)?;
            }
            self.bump();
        }

        self.parse_eof()
    }
}