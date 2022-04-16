use crate::cursor::Cursor;
use crate::token::*;
use ir::attempt::*;
use ir::hl_token::*;
use ir::ll_token::{DelimKind, LlTokenStream};

fn consume_bin_op(cursor: Cursor) -> Option<Result<HlToken, String>> {
    cursor
        .consume_next(is_bin_op)
        .map_to_token(bin_op_from_cursor)
}

fn consume_un_op(cursor: Cursor) -> Option<Result<HlToken, String>> {
    cursor
        .attempt_curr(is_un_op_precursor)
        .consume_next(is_un_op)
        .map_to_token(un_op_from_cursor)
}

fn consume_number(cursor: Cursor) -> Option<Result<HlToken, String>> {
    cursor.consume_next(is_num).map_to_token(num_from_cursor)
}

fn consume_ident(cursor: Cursor) -> Option<Result<HlToken, String>> {
    cursor
        .consume_next(is_ident)
        .map_to_token(ident_from_cursor)
}

fn consume_unkown(cursor: Cursor) -> Option<Result<HlToken, String>> {
    cursor.attempt_next(is_unknown).map_to_token(unkown_from_cursor)
}

fn consume_open_delim(cursor: Cursor) -> Option<DelimKind> {
    if let ir::ll_token::LlTokenKind::OpenDelim(delim_kind) = cursor.next() {
        Some(delim_kind)
    } else {
        None
    }
}

fn consume_close_delim(cursor: Cursor) -> Option<DelimKind> {
    if let ir::ll_token::LlTokenKind::CloseDelim(delim_kind) = cursor.next() {
        Some(delim_kind)
    } else {
        None
    }
}

const CONSUMERS: [fn(Cursor) -> Option<Result<HlToken, String>>; 5] =
    [consume_number, consume_ident, consume_un_op, consume_bin_op, consume_unkown];

pub fn tokenize(ll_stream: LlTokenStream) -> Result<HlTokenStream, String> {
    let mut delimiteds = Vec::new();
    let mut cursor = Cursor::new(ll_stream);
    let mut opened_delims = Vec::new();

    loop {
        if cursor.is_eof() {
            break;
        }

        if let Some(delim_kind) = consume_open_delim(cursor.clone()) {
            opened_delims.push(delim_kind);
            delimiteds.push(Vec::new());
            cursor.bump();
            continue;
        }

        if let Some(close_delim_kind) = consume_close_delim(cursor.clone()) {
            let open_delim_kind = if let Some(d_k) = opened_delims.pop() {
                d_k
            } else {
                return Err("Unopened delim".to_string());
            };

            if close_delim_kind != open_delim_kind {
                return Err("unopened delim".to_string());
            }

            let last_delimited = if let Some(delimited) = delimiteds.pop() {
                delimited
            } else {
                return Err("unopened delim".to_string());
            };

            let hl_token = HlToken::new(
                HlTokenKind::Delimited(open_delim_kind, last_delimited.into_iter()),
                cursor.span(),
            );

            let last_last_delimited = if let Some(mut delimited) = delimiteds.pop() {
                delimited.push(hl_token);
                delimited
            } else {
                vec![hl_token]
            };

            delimiteds.push(last_last_delimited);
            cursor.bump();
        }

        let mut last_delimited = if let Some(delimited) = delimiteds.pop() {
            delimited
        } else {
            vec![]
        };

        if let Attempt::Succeeded(_) = cursor.clone().attempt_next(is_whitespace) {
            cursor.bump();
        }

        if let Some(tok) = CONSUMERS.iter().find_map(|consume| consume(cursor.clone())) {
            cursor.bump();
            last_delimited.push(tok?);
        }

        delimiteds.push(last_delimited);
    }

    match delimiteds.pop() {
        Some(delimited) if delimiteds.len() == 0 => Ok(delimited.into_iter()),
        _ => Err("unopened delim".to_string()),
    }
}

/*
fn tokenize(ll_stream: LlTokenStream) {
    let mut open_delim_kind = None;
    let mut hl_tokens = Vec::new();
    let mut cursor = Cursor::new(ll_stream);

    while cursor.is_eof() {
        open_delim_kind = consume_open_delim(cursor.clone());


    }
}
*/
