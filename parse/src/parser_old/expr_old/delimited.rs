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

fn produce_delimited_expr<'stream>(
    token_stream: SharedTokenStream<'stream>,
) -> ProduceRes<Expr<'stream>> {
    let delim_kind = unwrap_enum!(token_stream.next().unwrap().val(), Token::OpenDelim);
    ProduceRes::Ok(Expr::new(ExprKind::Delimited(delim_kind.clone()), token_stream.into_cursor()))
}

fn produce_parse_stop<'stream>(
    expr_kind: &'stream ExprKind
    ) -> impl FnMut(SharedTokenStream<'stream>) -> ProduceRes<()> + Clone {
    move |mut token_stream| {
        let delim_kind = unwrap_enum!(token_stream.next().unwrap().val(), Token::CloseDelim);

        match expr_kind {
            ExprKind::Delimited(required_kind) if required_kind == delim_kind => ProduceRes::Ok(()),
            _ => ProduceRes::Err(ERR__UNOPENED_DELIMITED_BLOCK.to_owned()),
        }
    }
}

fn consume_delimited_start<'cursor>(cursor: TokenCursor<'cursor>) -> ConsumeRes<Expr<'cursor>> {
    cursor
        .consume_once()
        .look_at_first()
        .expect(is_any_open_delim)
        .produce(produce_delimited_expr)
}

fn consume_delimited_end<'cursor>(
    cursor: TokenCursor<'cursor>,
    expr_kind: &'cursor ExprKind,
) -> ConsumeRes<()> {
        cursor
            .look_at_first()
            .expect(is_any_close_delim)
            .produce(produce_parse_stop(delim_kind))
}

impl<'expr> Expr<'expr> {
    pub fn parse_delimited(&mut self) -> ParseRes<'expr> {
        match consume_delimited_start(self.cursor.clone()) {
            ConsumeRes::Ok(expr) => return ParseRes::Ok(ParseControlFlow::New(expr.val())),
            ConsumeRes::Err(err) => return ParseRes::Err(err),
            _ => ()
        }
    
        match consume_delimited_end(self.cursor.clone(), self.kind()) {
            ConsumeRes::Ok(_) => ParseRes::Ok(ParseControlFlow::Stop),
            ConsumeRes::Err(err) => ParseRes::Err(err),
            ConsumeRes::None => ParseRes::None,
        }

        /*
        try_modification!(self.cursor, check_close_delim);

        let (delim_kind, delim_span) = modify!(self.cursor, consume_delimited_start).to_tuple();
        self.cursor.cut_by_span(delim_span);

        let (expr, expr_span) = modify!(self.cursor, consume_delimited(delim_kind)).to_tuple();
        let expr_span = expr_span.increment_end().decrement_start();

        self.cursor.cut_by_span(expr_span);

        ParseRes::Ok(vec![expr])
        */
    }
}
