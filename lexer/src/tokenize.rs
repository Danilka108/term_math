use crate::token::*;
use ir::cursor::*;
use ir::span::*;
use ir::token::*;

const CONSUMERS: [for<'c> fn(StringCursor<'c>) -> Option<SpanWrapper<Token>>; 6] = [
    consume_number,
    consume_ident,
    consume_lit,
    consume_delim,
    consume_whitespace,
    consume_unknown,
];

fn consume_lit<'c>(cursor: StringCursor<'c>) -> Option<SpanWrapper<Token>> {
    cursor
        .attempt_next(is_lit)
        .consume_next(is_lit)
        .to_token(lit_from_string)
}

fn consume_delim<'c>(cursor: StringCursor<'c>) -> Option<SpanWrapper<Token>>{
    cursor
        .attempt_next(is_delim)
        .consume_next(is_delim)
        .to_token(delim_from_string)
}

fn consume_number<'c>(cursor: StringCursor<'c>) -> Option<SpanWrapper<Token>>{
    cursor
        .attempt_next(is_number_start)
        .consume_while(is_number)
        .to_token(num_from_string)
}

fn consume_ident<'c>(cursor: StringCursor<'c>) -> Option<SpanWrapper<Token>>{
    cursor
        .attempt_next(is_ident_start)
        .consume_while(is_ident)
        .to_token(ident_from_string)
}

fn consume_whitespace<'c>(cursor: StringCursor<'c>) -> Option<SpanWrapper<Token>>{
    cursor
        .attempt_next(is_whitespace)
        .consume_while(is_whitespace)
        .to_token(whitespace_from_string)
}

fn consume_unknown<'c>(cursor: StringCursor<'c>) -> Option<SpanWrapper<Token>> {
    cursor
        .attempt_next(is_unknown)
        .consume_while(is_unknown)
        .to_token(unkown_from_string)
}

pub fn tokenize(src: &str) -> TokenStream {
    let mut tokens = Vec::new();
    let mut cursor = src.into_cursor();


    while !cursor.is_eof() {
        let consumed_token = CONSUMERS.iter().find_map(|consume| consume(cursor.clone()));

        let token = if let Some(token) = consumed_token {
            token
        } else {
            continue;
        };

        cursor.cut_token(&token);
        tokens.push(token);
    }

    tokens.into_iter()
}
