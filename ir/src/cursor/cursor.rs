use crate::span::{Span, SpanWrapper};
use std::fmt::Debug;

pub type OriginalItem<CURSOR> = <<CURSOR as Cursor>::Original as Cursor>::Item;

pub trait Cursor
where
    Self: Clone + Debug,
{
    type Item: Clone + Debug;
    type Original: Cursor;

    fn bump(&mut self) -> Option<SpanWrapper<Self::Item>>;
    fn curr(&self) -> SpanWrapper<Self::Item>;

    fn original(&self) -> &Self::Original;

    fn next(&self) -> SpanWrapper<Self::Item> {
        self.clone().bump().unwrap_or(self.eof())
    }

    fn eof(&self) -> SpanWrapper<Self::Item> {
        let mut cloned = self.clone();

        while !cloned.is_eof() {
            cloned.bump();
        }

        cloned.curr()
    }

    fn is_eof(&self) -> bool {
        self.clone().bump().is_none()
    }

    fn span(&self) -> Span {
        let start = self.next().span().start();
        let end = self.eof().span().end();

        Span::new(start, end)
    }

    fn cut<VAL>(&mut self, span_wrapper: &SpanWrapper<VAL>)
    where
        VAL: Clone + Debug,
    {
        while self.span().start() < span_wrapper.borrow_span().end() && !self.is_eof() {
            self.bump();
        }
    }

    fn cut_by_span(&mut self, span: Span) {
        while self.span().start() < span.end() && !self.is_eof() {
            self.bump();
        }
    }

    fn collect<RETURN: FromCursor<Self::Item>>(self) -> RETURN {
        RETURN::from_cursor(self)
    }
}

pub trait FromCursor<ITEM> {
    fn from_cursor<CURSOR: IntoCursor<Item = ITEM>>(cursor: CURSOR) -> Self;
}

pub trait IntoCursor {
    type Item;

    type IntoCursor: Cursor<Item = Self::Item>;

    fn into_cursor(self) -> Self::IntoCursor;
}

impl<CURSOR: Cursor> IntoCursor for CURSOR {
    type Item = CURSOR::Item;
    type IntoCursor = CURSOR;

    fn into_cursor(self) -> Self::IntoCursor {
        self
    }
}
