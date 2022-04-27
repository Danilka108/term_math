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

pub fn consume_num<'cur>(cursor: TokenCursor<'cur>) -> ConsumeRes<String> {
    cursor
        .consume_once()
        .look_at_first()
        .expect(is_number)
        .produce(produce_num)
}

#[derive(Debug)]
pub struct NumNonterm<'nonterm> {
    val: SpanWrapper<String>,
    parent: &'nonterm mut BinOpNonterm<'nonterm>,
}

impl<'nonterm> Nonterm<'nonterm> for NumNonterm<'nonterm> {
    type Parent = BinOpNonterm<'nonterm>;
    type Next = Self;

    fn parse(
        parent: &'nonterm mut Self::Parent,
        cursor: TokenCursor<'nonterm>,
    ) -> ParseSignal<Vec<Self::Next>> {
        match consume_num(cursor) {
            ConsumeRes::Ok(val) => ParseSignal::New(vec![Self { parent, val }]),
            ConsumeRes::Err(err) => ParseSignal::Err(err),
            ConsumeRes::None => ParseSignal::None,
        }
    }

    fn attach(self, prev: Option<SpanWrapper<Node>>) -> Option<SpanWrapper<Node>> {
        self.parent.attach(Some(self.val.map(|v| Node::Num(v))))
    }
}
