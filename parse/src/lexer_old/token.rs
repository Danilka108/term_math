use ir::cursor::WrappedString;
use ir::cursor::prelude::Modification;
use ir::token::*;

pub fn is_lit(chr: char) -> bool {
    chr == '*' || chr == '/' || chr == '+' || chr == '-' || chr == ','
}

pub fn is_delim(chr: char) -> bool {
    chr == '(' || chr == ')' || chr == '{' || chr == '}' || chr == '[' || chr == ']'
}

pub fn is_number_start(chr: char) -> bool {
    chr.is_digit(10)
}

pub fn is_number(chr: char) -> bool {
    chr.is_digit(10) || chr.is_ascii_alphabetic() || chr == '.'
}

pub fn is_ident_start(chr: char) -> bool {
    chr.is_alphabetic()
}

pub fn is_ident(chr: char) -> bool {
    chr.is_alphabetic() || chr.is_digit(10)
}

pub fn is_whitespace(chr: char) -> bool {
    matches!(
        chr,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

pub fn is_unknown(chr: char) -> bool {
    !(is_number_start(chr)
        || is_number(chr)
        || is_ident_start(chr)
        || is_ident(chr)
        || is_lit(chr)
        || is_delim(chr)
        || is_whitespace(chr))
}

pub fn produce_lit(val: WrappedString) -> Modification<Token, ()> {
    let lit_kind = match val.as_str() {
        "*" => LitKind::Asterisk,
        "/" => LitKind::Slash,
        "+" => LitKind::Plus,
        "-" => LitKind::Hyphen,
        "," => LitKind::Comma,
        _ => return Modification::None,
    };

    Modification::Ok(Token::Lit(lit_kind))
}

pub fn produce_delim(val: WrappedString) -> Modification<Token, ()> {
    let delim_kind = match val.as_str() {
        "(" => Token::OpenDelim(DelimKind::Paren),
        ")" => Token::CloseDelim(DelimKind::Paren),
        "{" => Token::OpenDelim(DelimKind::Brace),
        "}" => Token::CloseDelim(DelimKind::Brace),
        "[" => Token::OpenDelim(DelimKind::Bracket),
        "]" => Token::CloseDelim(DelimKind::Bracket),
        _ => return Modification::None,
    };

    Modification::Ok(delim_kind)
}

pub fn produce_ident(val: WrappedString) -> Modification<Token, ()> {
    Modification::Ok(Token::Ident(val.into_inner()))
}

pub fn produce_num(val: WrappedString) -> Modification<Token, ()> {
    Modification::Ok(Token::Num(val.into_inner()))
}

pub fn produce_whitespace(_: WrappedString) -> Modification<Token, ()> {
    Modification::Ok(Token::Whitespace)
}

pub fn produce_unknown(_: WrappedString) -> Modification<Token, ()> {
    Modification::Ok(Token::Unknown)
}
