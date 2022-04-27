use super::errors::ERR__MISSIGN_OPERATOR;
use super::*;
use ir::ast::*;
use ir::cursor::prelude::*;
use ir::cursor::TokenCursor;
use ir::token::*;
use util::*;

fn is_number<'tok>(token: &'tok Token) -> bool {
    matches!(token, &Token::Num(_))
}

fn produce_num<'stream>(mut token_stream: SharedTokenStream<'stream>) -> ProduceRes<String> {
    match token_stream.next().unwrap().val() {
        Token::Num(v) => ProduceRes::Ok(v.to_owned()),
        _ => return ProduceRes::None,
    }
}

fn is_left_arg_delimiter_or_eof<'token>(expr_kind: &'token ExprKind) -> impl FnMut(&'token Token) -> bool + Clone {
    move |token| {
        (matches!(token, Token::Eof) || is_left_arg_delimiter(token)) && matches!(expr_kind, ExprKind::FnArg)
    }
}

fn is_right_arg_delimiter_or_eof<'token>(expr_kind: &'token ExprKind) -> impl FnMut(&'token Token) -> bool + Clone {
    move |token| {
        (matches!(token, Token::Eof) || is_right_arg_delimiter(token)) && matches!(expr_kind, ExprKind::FnArg)
    }
}

pub fn consume_num<'cur>(cursor: TokenCursor<'cur>, expr_kind: &ExprKind) -> ConsumeRes<String> {
    let is_arg = cursor
        .clone()
        .consume_once()
        .look_at_first()
        .expect(is_number)
        .look_at_curr()
        .require(is_left_arg_delimiter_or_eof(expr_kind), ERR__MISSIGN_OPERATOR.to_owned())
        .look_at_second()
        .require(is_left_arg_delimiter_or_eof(expr_kind), ERR__MISSIGN_OPERATOR.to_owned())
        .produce(produce_num);

    let is_present_right_operator = cursor
        .clone()
        .consume_once()
        .look_at_first()
        .expect(is_number)
        .look_at_second()
        .require(is_bin_op, ERR__MISSIGN_OPERATOR.to_owned())
        .produce(produce_num);

    let is_present_left_operator = cursor
        .consume_once()
        .look_at_first()
        .expect(is_number)
        .look_at_curr()
        .require(is_op, ERR__MISSIGN_OPERATOR.to_owned())
        .produce(produce_num);

    is_present_left_operator
        .or(is_present_right_operator)
        .or(is_arg)
}

impl<'expr> Expr<'expr> {
    pub fn parse_num(&mut self) -> ParseRes<'expr> {
        let (num, span) = modify!(consume_num(self.cursor.clone(), self.kind())).to_tuple();
        self.cursor.cut_by_span(span.clone());

        let node = SpanWrapper::new(Node::Num(num), span);
        self.nodes.push(node);

        ParseRes::None
    }
}
