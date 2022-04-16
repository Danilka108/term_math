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

pub fn lit_from_string(val: String) -> Option<Token> {
    let lit_kind = match val.as_str() {
        "*" => LitKind::Asterisk,
        "/" => LitKind::Slash,
        "+" => LitKind::Plus,
        "-" => LitKind::Hyphen,
        "," => LitKind::Comma,
        _ => return None,
    };

    Some(Token::Lit(lit_kind))
}

pub fn delim_from_string(val: String) -> Option<Token> {
    let delim_kind = match val.as_str() {
        "(" => Token::OpenDelim(DelimKind::Paren),
        ")" => Token::CloseDelim(DelimKind::Paren),
        "{" => Token::OpenDelim(DelimKind::Brace),
        "}" => Token::CloseDelim(DelimKind::Brace),
        "[" => Token::OpenDelim(DelimKind::Bracket),
        "]" => Token::CloseDelim(DelimKind::Bracket),
        _ => return None,
    };

    Some(delim_kind)
}

pub fn ident_from_string(val: String) -> Option<Token> {
    Some(Token::Ident(val))
}

pub fn num_from_string(val: String) -> Option<Token> {
    Some(Token::Num(val))
}

pub fn whitespace_from_string(val: String) -> Option<Token> {
    Some(Token::Whitespace)
}

pub fn unkown_from_string(val: String) -> Option<Token> {
    Some(Token::Unknown)
}
