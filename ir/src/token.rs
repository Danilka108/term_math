use crate::span::SpanWrapper;
use crate::cursor::{Cursor, FromCursor, IntoCursor};
use std::str::Chars;

const EOF_CHAR: char = '\0';

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
}

pub type TokenStream = std::vec::IntoIter<SpanWrapper<Token>>;

#[derive(Debug, Clone)]
pub struct StringCursor<'c> {
    curr: char,
    chars: Chars<'c>,
    offset: usize,
}

impl<'c> Cursor<Token> for StringCursor<'c> {
    type Item = char;

    fn bump(&mut self) -> Option<Self::Item> {
        let next_char = self.chars.next();

        self.offset += 1;
        self.curr = next_char.unwrap_or(EOF_CHAR);

        next_char
    }

    fn curr(&self) -> Self::Item {
        self.curr
    }

    fn start(&self) -> usize {
        self.offset
    }

    fn end(&self) -> usize {
        self.offset + self.chars.clone().count()
    }
}

impl<'c> IntoCursor<Token> for &'c str {
    type Item = char;
    type IntoCursor = StringCursor<'c>;

    fn into_cursor(self) -> Self::IntoCursor {
        StringCursor {
            chars: self.chars(),
            offset: 0,
            curr: EOF_CHAR,
        }
    }
}

impl<'c> FromCursor<Token, char> for String {
    fn from_cursor<C: IntoCursor<Token, Item = char>>(c: C) -> Self {
        let mut val = String::new();
        let mut cursor = c.into_cursor();

        while let Some(chr) = cursor.bump() {
            val.push(chr);
        }

        val
    }
}
