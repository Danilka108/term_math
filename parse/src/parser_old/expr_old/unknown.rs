use super::errors::ERR__UNKNOWN_SYMBOLS;
use ir::token::*;
use ir::cursor::prelude::*;
use ir::cursor::TokenCursor;
use super::*;
use util::*;

fn is_unknown(token: &Token) -> bool {
    matches!(token, Token::Unknown)
}

fn consume_unknown<'cursor>(cursor: TokenCursor<'cursor>) -> ConsumeRes<()> {
    cursor
        .consume_once()
        .look_at_first()
        .throw_if(is_unknown, ERR__UNKNOWN_SYMBOLS.to_owned())
        .produce(|_: SharedTokenStream<'cursor>| Modification::None)
}

impl<'expr> Expr<'expr> {
    pub fn parse_unknown(&mut self) -> ParseRes<'expr> {
        modify!(self.cursor, consume_unknown).to_tuple();
        ParseRes::None
    }
}
