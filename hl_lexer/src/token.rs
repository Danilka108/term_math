use number::Dec64;

use ir::ast::{BinOpKind, UnOpKind};
use ir::hl_token::*;
use ir::ll_token::{DelimKind, LitKind, LlTokenKind};

use crate::cursor::Cursor;

pub fn is_bin_op(kind: LlTokenKind) -> bool {
    matches!(
        kind,
        LlTokenKind::Lit(LitKind::Asterisk | LitKind::Slash | LitKind::Plus | LitKind::Hyphen)
    )
}

pub fn is_un_op_precursor(kind: LlTokenKind) -> bool {
    matches!(
        kind,
        LlTokenKind::Lit(LitKind::Comma) | LlTokenKind::CloseDelim(_) | LlTokenKind::Eof
    )
}

pub fn is_un_op(kind: LlTokenKind) -> bool {
    matches!(kind, LlTokenKind::Lit(LitKind::Hyphen))
}

pub fn is_num(kind: LlTokenKind) -> bool {
    matches!(kind, LlTokenKind::Num(_))
}

pub fn is_ident(kind: LlTokenKind) -> bool {
    matches!(kind, LlTokenKind::Ident(_))
}

pub fn is_comma(kind: LlTokenKind) -> bool {
    kind == LlTokenKind::Lit(LitKind::Comma)
}

pub fn is_whitespace(kind: LlTokenKind) -> bool {
    kind == LlTokenKind::Whitespace
}

pub fn is_unknown(kind: LlTokenKind) -> bool {
    kind == LlTokenKind::Unknown
}

pub fn bin_op_from_cursor(cursor: Cursor) -> Option<Result<HlTokenKind, String>> {
    let lit_kind = match cursor.as_ll_stream().next()?.val() {
        LlTokenKind::Lit(lit_kind) => lit_kind,
        _ => return None,
    };

    let kind = match lit_kind {
        LitKind::Asterisk => HlTokenKind::BinOp(BinOpKind::Mul),
        LitKind::Slash => HlTokenKind::BinOp(BinOpKind::Div),
        LitKind::Plus => HlTokenKind::BinOp(BinOpKind::Add),
        LitKind::Hyphen => HlTokenKind::BinOp(BinOpKind::Sub),
        _ => return None,
    };

    Some(Ok(kind))
}

pub fn un_op_from_cursor(cursor: Cursor) -> Option<Result<HlTokenKind, String>> {
    let lit_kind = match cursor.as_ll_stream().next()?.val() {
        LlTokenKind::Lit(lit_kind) => lit_kind,
        _ => return None,
    };

    let kind = match lit_kind {
        LitKind::Hyphen => HlTokenKind::UnOp(UnOpKind::Neg),
        _ => return None,
    };

    Some(Ok(kind))
}

pub fn num_from_cursor(cursor: Cursor) -> Option<Result<HlTokenKind, String>> {
    match cursor.as_ll_stream().next()?.val() {
        LlTokenKind::Num(val) => Some(Ok(HlTokenKind::Num(val))),
        _ => None,
    }
}

pub fn ident_from_cursor(cursor: Cursor) -> Option<Result<HlTokenKind, String>> {
    match cursor.as_ll_stream().next()?.val() {
        LlTokenKind::Ident(val) => Some(Ok(HlTokenKind::Ident(val))),
        _ => None,
    }
}

pub fn comma_from_cursor(cursor: Cursor) -> Option<Result<HlTokenKind, String>> {
    match cursor.as_ll_stream().next()?.val() {
        LlTokenKind::Lit(LitKind::Comma) => Some(Ok(HlTokenKind::Comma)),
        _ => None,
    }
}

pub fn unkown_from_cursor(cursor: Cursor) -> Option<Result<HlTokenKind, String>> {
    match cursor.as_ll_stream().next()?.val() {
        LlTokenKind::Unknown => Some(Err("Unkown symbols".to_string())),
        _ => None,
    }
}
