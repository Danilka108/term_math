use super::token::*;
use ir::cursor::prelude::*;
use ir::cursor::{StringCursor, WrappedStr};
use ir::span::*;
use ir::token::*;

const CONSUMERS: [for<'c> fn(StringCursor<'c>) -> Modification<SpanWrapper<Token>, SpanWrapper<()>>; 6] = [
    consume_number,
    consume_ident,
    consume_lit,
    consume_delim,
    consume_whitespace,
    consume_unknown,
];

fn consume_lit<'c>(cursor: StringCursor<'c>) -> Modification<SpanWrapper<Token>, SpanWrapper<()>> {
    cursor
        .look_at_first()
        .expect(is_lit)
        .consume_once()
        .produce(produce_lit)
}

fn consume_delim<'c>(cursor: StringCursor<'c>) -> Modification<SpanWrapper<Token>, SpanWrapper<()>> {
    cursor.look_at_first().expect(is_delim).consume_once().produce(produce_delim)
}

fn consume_number<'c>(cursor: StringCursor<'c>) -> Modification<SpanWrapper<Token>, SpanWrapper<()>> {
    cursor
        .look_at_first()
        .expect(is_number_start)
        .consume_while(is_number)
        .produce(produce_num)
}

fn consume_ident<'c>(cursor: StringCursor<'c>) -> Modification<SpanWrapper<Token>, SpanWrapper<()>> {
    cursor
        .look_at_first()
        .expect(is_ident_start)
        .consume_while(is_ident)
        .produce(produce_ident)
}

fn consume_whitespace<'c>(cursor: StringCursor<'c>) -> Modification<SpanWrapper<Token>, SpanWrapper<()>> {
    cursor
        .consume_while(is_whitespace)
        .produce(produce_whitespace)
}

fn consume_unknown<'c>(cursor: StringCursor<'c>) -> Modification<SpanWrapper<Token>, SpanWrapper<()>> {
    cursor.consume_while(is_unknown).produce(produce_unknown)
}

pub fn tokenize(src: &str) -> TokenStream {
    let mut tokens = Vec::new();
    let mut cursor = WrappedStr::wrap(src).into_cursor();

    while !cursor.is_eof() {
        let consumed_token = CONSUMERS.iter().find_map(|consume| consume(cursor.clone()).unwrap_as_option());

        let token = if let Some(token) = consumed_token {
            token
        } else {
            continue;
        };

        cursor.cut(&token);
        tokens.push(token);
    }

    let eof_span = Span::new(src.len(), src.len() + 1);
    let eof = SpanWrapper::new(Token::Eof, eof_span);
    tokens.push(eof);

    tokens.into_iter()
}
