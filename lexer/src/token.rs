use ast::span::Span;
use number::Dec64;

#[derive(Debug, Clone)]
pub enum LitKind {
    Asterisk,
    Slash,
    Plus,
    Hyphen,
    Comma,
}

#[derive(Debug, Clone)]
pub enum DelimKind {
    Paren,
    Brace,
    Bracket,
}

#[derive(Debug, Clone)]
pub enum TokenKind<'s> {
    Lit(LitKind),
    OpenDelim(DelimKind),
    CloseDelim(DelimKind),
    Ident(&'s str),
    Num(Dec64),
    Err(String),
    Eof,
}

pub fn lit_from_char<'v>(val: &'v str) -> Option<TokenKind<'v>> {
    let lit_kind = match val {
        "*" => LitKind::Asterisk,
        "/" => LitKind::Slash,
        "+" => LitKind::Plus,
        "-" => LitKind::Hyphen,
        "," => LitKind::Comma,
        _ => return None,
    };

    Some(TokenKind::Lit(lit_kind))
}

#[derive(Debug, Clone)]
pub struct Token<'k> {
    pub kind: TokenKind<'k>,
    pub span: Span,
}

impl<'k> Token<'k> {
    pub fn new(kind: TokenKind<'k>, span: Span) -> Self {
        Self { kind, span }
    }
}

pub fn delim_from_char<'v>(val: &'v str) -> Option<TokenKind<'v>> {
    let delim_kind = match val {
        "(" => TokenKind::OpenDelim(DelimKind::Paren),
        ")" => TokenKind::CloseDelim(DelimKind::Paren),
        "{" => TokenKind::OpenDelim(DelimKind::Brace),
        "}" => TokenKind::CloseDelim(DelimKind::Brace),
        "[" => TokenKind::OpenDelim(DelimKind::Bracket),
        "]" => TokenKind::CloseDelim(DelimKind::Bracket),
        _ => return None,
    };

    Some(delim_kind)
}

pub fn ident_from_str<'v>(val: &'v str) -> Option<TokenKind<'v>> {
    Some(TokenKind::Ident(val))
}

pub fn num_from_str<'v>(val: &'v str) -> Option<TokenKind<'v>> {
    let res = match Dec64::try_from(val) {
        Ok(num) => TokenKind::Num(num),
        Err(err) => TokenKind::Err("Invalid number: ".to_string() + &err.to_string()),
    };

    Some(res)
}
