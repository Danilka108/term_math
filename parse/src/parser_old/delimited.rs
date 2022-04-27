use super::errors::{
    ERR__EMPTY_DELIMITED_BLOCK, ERR__UNCLOSED_DELIMITED_BLOCK, ERR__UNOPENED_DELIMITED_BLOCK,
};
use super::*;
use ir::cursor::prelude::*;
use ir::cursor::TokenCursor;
use ir::token::*;
use util::*;

pub fn is_any_open_delim<'tok>(token: &Token) -> bool {
    matches!(&token, Token::OpenDelim(_))
}

pub fn is_any_close_delim<'tok>(token: &Token) -> bool {
    matches!(&token, Token::CloseDelim(_))
}

fn is_open_delim<'tok>(required_kind: &'tok DelimKind) -> impl FnMut(&'tok Token) -> bool + Clone {
    move |token| matches!(&token, Token::OpenDelim(kind) if kind == required_kind)
}

fn is_close_delim<'tok>(required_kind: &'tok DelimKind) -> impl FnMut(&'tok Token) -> bool + Clone {
    move |token| matches!(&token, Token::CloseDelim(kind) if kind == required_kind)
}

fn produce_open_delim<'stream>(token_stream: SharedTokenStream<'stream>) -> ProduceRes<DelimKind> {
    ProduceRes::Ok(unwrap_enum!(
        token_stream.next().unwrap().val().clone(),
        Token::OpenDelim
    ))
}

fn produce_close_delim<'stream>(token_stream: SharedTokenStream<'stream>) -> ProduceRes<DelimKind> {
    ProduceRes::Ok(unwrap_enum!(
        token_stream.next().unwrap().val().clone(),
        Token::CloseDelim
    ))
}

fn consume_delimited_start<'cursor>(cursor: TokenCursor<'cursor>) -> ConsumeRes<DelimKind> {
    cursor
        .consume_once()
        .look_at_first()
        .expect(is_any_open_delim)
        .produce(produce_open_delim)
}

fn consume_delimited_end<'cursor>(cursor: TokenCursor<'cursor>) -> ConsumeRes<DelimKind> {
    cursor
        .consume_once()
        .look_at_first()
        .expect(is_any_close_delim)
        .produce(produce_close_delim)
}

pub struct DelimitedExpr<'frame, TOK, NODE, CUR, FRAME>
where
    TOK: 'frame,
    NODE: Clone + Debug,
    CUR: Cursor<Item = &'frame TOK>,
    FRAME: ParserFrame<'frame, TOK, NODE, CUR>,
{
    frame: FRAME,
    _phantom: (PhantomData<'frame>, PhantomData<TOK>, PhantomData<>
}

/*
impl<'parser> Parser<'parser> {
    /// End of parsing hook!
    fn check_nonclosed_delims(&mut self) -> Result<(), SpanWrapper<String>> {
        match self.buff.pop() {
            Some(elem) if matches!(elem.borrow_val(), &BuffElem::Delim(_)) => Err(
                SpanWrapper::new(ERR__UNCLOSED_DELIMITED_BLOCK.to_owned(), elem.span()),
            ),
            _ => Ok(()),
        }
    }

    fn parse_delimited_end(&mut self) -> Result<(), SpanWrapper<String>> {
        let (required_delim_kind, span) = match consume_delimited_end(self.cursor.clone()) {
            ConsumeRes::Ok(res) => res.to_tuple(),
            ConsumeRes::Err(err) => return Err(err),
            ConsumeRes::None => return Ok(()),
        };

        self.cursor.cut_by_span(span.clone());
        let node = match self.collect() {
            Some(val) => val,
            None => {
                return Err(SpanWrapper::new(
                    ERR__EMPTY_DELIMITED_BLOCK.to_owned(),
                    span,
                ))
            }
        };

        match self.buff.pop() {
            Some(elem) if matches!(elem.val(), BuffElem::Delim(delim_kind) if delim_kind == required_delim_kind) => {
                Ok(())
            }
            _ => Err(SpanWrapper::new(
                ERR__UNOPENED_DELIMITED_BLOCK.to_owned(),
                span,
            )),
        }
    }

    pub fn parse_delimited_start(&mut self) -> Result<(), SpanWrapper<String>> {
        let (delim_kind, span) = match consume_delimited_start(self.cursor.clone()) {
            ConsumeRes::Ok(res) => res.to_tuple(),
            ConsumeRes::Err(err) => return Err(err),
            ConsumeRes::None => return Ok(()),
        };

        self.cursor.cut_by_span(span.clone());
        self.buff
            .push(SpanWrapper::new(BuffElem::Delim(delim_kind), span));

        Ok(())
    }
}*/
