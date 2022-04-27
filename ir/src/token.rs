use crate::span::{SharedSpanWrapper, SpanWrapper};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LitKind {
    Asterisk,
    Slash,
    Plus,
    Hyphen,
    Comma,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelimKind {
    Paren,
    Brace,
    Bracket,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Lit(LitKind),
    OpenDelim(DelimKind),
    CloseDelim(DelimKind),
    Ident(String),
    Num(String),
    Whitespace,
    Unknown,
    Eof,
}

pub type TokenStream = std::vec::IntoIter<SpanWrapper<Token>>;

pub trait ToShared<'t> {
    fn to_shared_stream(&'t self) -> SharedTokenStream<'t>;
}

impl<'t> ToShared<'t> for TokenStream {
    fn to_shared_stream(&'t self) -> SharedTokenStream<'t> {
        self.as_slice()
            .into_iter()
            .map(|w| SharedSpanWrapper::from(w))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

pub type SharedTokenStream<'t> = std::vec::IntoIter<SpanWrapper<&'t Token>>;
