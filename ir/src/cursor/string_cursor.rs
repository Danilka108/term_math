use super::cursor::{Cursor, FromCursor, IntoCursor};
use super::wrapped::{WrappedStr, WrappedString};
use crate::span::{Span, SpanWrapper};
use std::str::Chars;

pub const EOF_CHAR: char = '\0';

#[derive(Debug, Clone)]
pub struct StringCursor<'c> {
    curr: char,
    chars: Chars<'c>,
    offset: usize,
}

impl<'c> Cursor for StringCursor<'c> {
    type Item = char;
    type Original = Self;

    fn bump(&mut self) -> Option<SpanWrapper<Self::Item>> {
        let next_char = self.chars.next()?;
        let span = Span::new(self.offset, self.offset + 1);

        self.offset += 1;
        self.curr = next_char;

        Some(SpanWrapper::new(next_char, span))
    }

    fn curr(&self) -> SpanWrapper<Self::Item> {
        let span_start = self.offset.checked_sub(1).unwrap_or(0);
        let span_end = self.offset;
        let span = Span::new(span_start, span_end);

        SpanWrapper::new(self.curr, span)
    }

    fn original(&self) -> &Self::Original {
        &self
    }
}

impl<'c> IntoCursor for WrappedStr<'c> {
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

impl<'c> FromCursor<char> for WrappedString {
    fn from_cursor<C: IntoCursor<Item = char>>(c: C) -> Self {
        let mut val = String::new();
        let mut cursor = c.into_cursor();

        while let Some(chr) = cursor.bump() {
            val.push(chr.val());
        }

        WrappedString::wrap(val)
    }
}
