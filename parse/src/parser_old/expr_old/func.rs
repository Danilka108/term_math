use super::errors::{
    ERR__EMPTY_ARG, ERR__MISSIGN_ARGS_BLOCK_END, ERR__MISSIGN_ARGS_BLOCK_START,
    ERR__MISSIGN_LEFT_ARG_DELIM, ERR__MISSIGN_RIGHT_ARG_DELIM,
};
use super::*;
use ir::cursor::prelude::*;
use ir::cursor::TokenCursor;
use ir::span::*;
use ir::token::*;
use util::*;

fn is_ident(token: &Token) -> bool {
    matches!(token, Token::Ident(_))
}

pub fn is_left_arg_delimiter<'tok>(token: &'tok Token) -> bool {
    matches!(&token, Token::Lit(LitKind::Comma)) || matches!(token, Token::OpenDelim(DelimKind::Paren))
}

pub fn is_right_arg_delimiter<'tok>(token: &'tok Token) -> bool {
    matches!(&token, Token::Lit(LitKind::Comma)) || matches!(token, Token::CloseDelim(DelimKind::Paren))
}

fn is_arg_start<'tok>(is_first: bool) -> impl FnMut(&'tok Token) -> bool + Clone {
    move |token: &'tok Token| {
        !is_first && matches!(&token, Token::Lit(LitKind::Comma))
            || is_first && matches!(token, Token::OpenDelim(DelimKind::Paren))
    }
}

fn is_arg_end<'tok>(is_last: bool) -> impl FnMut(&'tok Token) -> bool + Clone {
    move |token: &'tok Token| -> bool {
        !is_last && matches!(&token, Token::Lit(LitKind::Comma))
            || is_last && matches!(token, Token::CloseDelim(DelimKind::Paren))
    }
}

fn produce_ident<'stream>(mut token_stream: SharedTokenStream<'stream>) -> ProduceRes<String> {
    match token_stream.next().unwrap().val() {
        Token::Ident(val) => ProduceRes::Ok(val.to_owned()),
        _ => ProduceRes::None,
    }
}

fn produce_arg<'stream>(token_stream: SharedTokenStream<'stream>) -> ProduceRes<Expr<'stream>> {
    ProduceRes::Ok(Expr::new(ExprKind::FnArg, token_stream.into_cursor()))
}

fn consume_ident<'cursor>(cursor: TokenCursor<'cursor>) -> ConsumeRes<String> {
    cursor
        .look_at_first()
        .expect(is_ident)
        .consume_once()
        .produce(produce_ident)
}

fn calc_skip<'cursor>(mut cursor: TokenCursor<'cursor>, is_first: bool, is_last: bool) -> usize {
    cursor.bump();
    cursor.count_to(is_arg_start(is_first), is_arg_end(is_last))
}

fn consume_single_arg<'cursor>(cursor: TokenCursor<'cursor>) -> ConsumeRes<Expr<'cursor>> {
    let skip = calc_skip(cursor.clone(), true, true);

    cursor
        .look_at_curr()
        .require(is_arg_start(true), ERR__MISSIGN_ARGS_BLOCK_START.to_owned())
        .consume_to(
            is_arg_end(true),
            skip,
            ERR__MISSIGN_ARGS_BLOCK_END.to_owned(),
        )
        .produce_not_empty(produce_arg, ERR__EMPTY_ARG.to_owned())
}

fn consume_first_arg<'cursor>(cursor: TokenCursor<'cursor>) -> ConsumeRes<Expr<'cursor>> {
    let skip = calc_skip(cursor.clone(), true, false);

    cursor
        .look_at_curr()
        .require(is_arg_start(true), ERR__MISSIGN_ARGS_BLOCK_START.to_owned())
        .consume_to(
            is_arg_end(false),
            skip,
            ERR__MISSIGN_RIGHT_ARG_DELIM.to_owned(),
        )
        .produce_not_empty(produce_arg, ERR__EMPTY_ARG.to_string())
}

fn consume_midle_arg<'cursor>(cursor: TokenCursor<'cursor>) -> ConsumeRes<Expr<'cursor>> {
    let skip = calc_skip(cursor.clone());

    cursor
        .look_at_curr()
        .expect(is_arg_start(false))
        .consume_to(
            is_arg_end(false),
            skip,
            ERR__MISSIGN_RIGHT_ARG_DELIM.to_owned(),
        )
        .produce_not_empty(produce_arg, ERR__EMPTY_ARG.to_string())
        .nonerr()
}

fn consume_last_arg<'cursor>(cursor: TokenCursor<'cursor>) -> ConsumeRes<Expr<'cursor>> {
    let skip = calc_skip(cursor.clone());

    cursor
        .look_at_curr()
        .require(is_arg_start(false), ERR__MISSIGN_LEFT_ARG_DELIM.to_owned())
        .consume_to(
            is_arg_end(true),
            skip,
            ERR__MISSIGN_ARGS_BLOCK_END.to_owned(),
        )
        .produce_not_empty(produce_arg, ERR__EMPTY_ARG.to_string())
}

impl<'expr> Expr<'expr> {
    fn parse_unary_fn(&mut self, fn_name: String, ident_span: Span) -> ParseRes<'expr> {
        let (arg, arg_span) = match consume_single_arg(self.cursor.clone()) {
            ConsumeRes::Ok(single_arg) => single_arg.to_tuple(),
            ConsumeRes::Err(err) => return ParseRes::Err(err),
            ConsumeRes::None => {
                return ParseRes::Err(SpanWrapper::new(
                    ERR__MISSIGN_ARGS_BLOCK_START.to_owned(),
                    ident_span,
                ))
            }
        };

        self.cursor.cut_by_span(arg_span.clone());
        self.cursor.bump();

        let span = [ident_span, arg_span].concat_span();
        self.buffer
            .push(SpanWrapper::new(BuffElement::Fn(fn_name), span));

        ParseRes::Ok(vec![arg])
    }

    fn parse_nonunary_fn(&mut self, fn_name: String, ident_span: Span) -> ParseRes<'expr> {
        let mut args = Vec::new();

        let (first_arg, first_arg_span) = match consume_first_arg(self.cursor.clone()) {
            ConsumeRes::Ok(first_arg) => first_arg.to_tuple(),
            ConsumeRes::Err(err) => return ParseRes::Err(err),
            _ => {
                return ParseRes::Err(SpanWrapper::new(
                    ERR__MISSIGN_ARGS_BLOCK_START.to_owned(),
                    ident_span,
                ))
            }
        };

        args.push(first_arg);

        let mut span = [ident_span, first_arg_span].concat_span();
        self.cursor.cut_by_span(span.clone());
        self.cursor.bump();

        while let ConsumeRes::Ok(middle_arg) = try_modification!(self.cursor, consume_midle_arg) {
            let (middle_arg, middle_arg_span) = middle_arg.to_tuple();

            args.push(middle_arg);
            span = [span, middle_arg_span].concat_span();

            self.cursor.cut_by_span(span.clone());
            self.cursor.bump();
        }

        let (last_arg, last_arg_span) = match consume_last_arg(self.cursor.clone()) {
            ConsumeRes::Ok(last_arg) => last_arg.to_tuple(),
            ConsumeRes::Err(err) => return ParseRes::Err(err),
            _ => {
                return ParseRes::Err(SpanWrapper::new(
                    ERR__MISSIGN_ARGS_BLOCK_END.to_owned(),
                    span,
                ))
            }
        };

        args.push(last_arg);
        span = [span, last_arg_span].concat_span();
        self.cursor.cut_by_span(span.clone());
        self.cursor.bump();

        dbg!(args.clone());
        self.buffer
            .push(SpanWrapper::new(BuffElement::Fn(fn_name), span));
        ParseRes::Ok(args)
    }

    pub fn parse_fn(&mut self) -> ParseRes<'expr> {
        let (fn_name, ident_span) = modify!(self.cursor, consume_ident).to_tuple();
        self.cursor.cut_by_span(ident_span.clone());
        self.cursor.bump();

        self.parse_nonunary_fn(fn_name.clone(), ident_span.clone()).or(self.parse_unary_fn(fn_name, ident_span))
    }
}
