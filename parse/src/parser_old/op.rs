use super::errors::{ERR__INVALID_LEFT_OPERAND, ERR__INVALID_RIGHT_OPERAND};
use super::*;
use ir::ast::*;
use ir::cursor::prelude::*;
use ir::cursor::TokenCursor;
use ir::token::*;
use util::*;

pub fn is_bin_op<'tok>(token: &'tok Token) -> bool {
    matches!(
        token,
        Token::Lit(LitKind::Plus | LitKind::Asterisk | LitKind::Hyphen | LitKind::Slash)
    )
}

fn is_un_op<'tok>(token: &'tok Token) -> bool {
    matches!(token, Token::Lit(LitKind::Hyphen))
}

pub fn is_op<'tok>(token: &'tok Token) -> bool {
    is_un_op(token) || is_bin_op(token)
}

fn is_right_operand<'tok>(token: &'tok Token) -> bool {
    matches!(token, Token::Num(_) | Token::OpenDelim(_) | Token::Ident(_))
}

fn is_left_operand<'tok>(token: &'tok Token) -> bool {
    matches!(token, Token::Num(_) | Token::CloseDelim(_))
}

fn produce_bin_op<'stream>(
    mut token_stream: SharedTokenStream<'stream>,
) -> ProduceRes<BinOpKind> {
    let kind = match token_stream.next().unwrap().val() {
        Token::Lit(LitKind::Plus) => BinOpKind::Add,
        Token::Lit(LitKind::Hyphen) => BinOpKind::Sub,
        Token::Lit(LitKind::Asterisk) => BinOpKind::Mul,
        Token::Lit(LitKind::Slash) => BinOpKind::Div,
        _ => return ProduceRes::None,
    };

    ProduceRes::Ok(kind)
}

fn produce_un_op<'stream>(mut token_stream: SharedTokenStream<'stream>) -> ProduceRes<UnOpKind> {
    match token_stream.next().unwrap().val() {
        Token::Lit(LitKind::Hyphen) => ProduceRes::Ok(UnOpKind::Neg),
        _ => ProduceRes::None,
    }
}

fn consume_bin_op<'cur>(cursor: TokenCursor<'cur>) -> ConsumeRes<BinOpKind> {
    cursor
        .consume_once()
        .look_at_first()
        .expect(is_bin_op)
        .look_at_second()
        .require(is_right_operand, ERR__INVALID_RIGHT_OPERAND.to_owned())
        .look_at_curr()
        .require(is_left_operand, ERR__INVALID_LEFT_OPERAND.to_owned())
        .produce(produce_bin_op)
}

fn consume_un_op<'cur>(cursor: TokenCursor<'cur>) -> ConsumeRes<UnOpKind> {
    cursor
        .consume_once()
        .look_at_first()
        .expect(is_un_op)
        .look_at_second()
        .require(is_right_operand, ERR__INVALID_RIGHT_OPERAND.to_owned())
        .produce(produce_un_op)
}
