use std::{fmt::Debug, marker::PhantomData};

use crate::span::{Span, SpanWrapper};

pub trait Cursor<T>
where
    T: Debug + Clone,
    Self: Clone + Debug,
{
    type Item: Clone;

    fn bump(&mut self) -> Option<Self::Item>;
    fn curr(&self) -> Self::Item;
    fn start(&self) -> usize;
    fn end(&self) -> usize;

    fn is_eof(&self) -> bool {
        self.clone().bump().is_none()
    }

    fn eof(&self) -> Self::Item {
        let mut cloned = self.clone();

        while !cloned.is_eof() {
            cloned.bump();
        }

        cloned.curr()
    }

    fn cut_token(&mut self, token: &SpanWrapper<T>) {
        while self.start() < token.span().end() && !self.is_eof() {
            self.bump();
        }
    }

    fn next(&self) -> Self::Item {
        self.clone().bump().unwrap_or(self.eof())
    }

    fn span(&self) -> Span {
        Span::new(self.start(), self.end())
    }

    fn collect<R: FromCursor<T, Self::Item>>(self) -> R {
        R::from_cursor(self)
    }
}

pub trait FromCursor<T: Clone + Debug, A> {
    fn from_cursor<C: IntoCursor<T, Item = A>>(cursor: C) -> Self;
}

pub trait IntoCursor<T: Clone + Debug> {
    type Item;

    type IntoCursor: Cursor<T, Item = Self::Item>;

    fn into_cursor(self) -> Self::IntoCursor;
}

impl<T: Debug + Clone, C: Cursor<T>> IntoCursor<T> for C {
    type Item = C::Item;
    type IntoCursor = C;

    fn into_cursor(self) -> Self::IntoCursor {
        self
    }
}

pub trait AttemptNext<T: Clone + Debug, C: Cursor<T>, F> {
    fn attempt_next(self, predicate: F) -> Option<C>;
}

pub trait ToToken<R: FromCursor<T, C::Item>, T: Clone + Debug, C: Cursor<T>, F> {
    fn to_token(self, predicate: F) -> Option<SpanWrapper<T>>;
}

pub trait ConsumeNext<T: Clone + Debug, C: Cursor<T>, F> {
    type Output;

    fn consume_next(self, predicate: F) -> Self::Output;
}

pub trait ConsumeWhile<T: Clone + Debug, C: Cursor<T>, F> {
    type Output;

    fn consume_while(self, predicate: F) -> Self::Output;
}

#[derive(Clone)]
pub struct Consume<T: Clone + Debug, C: Cursor<T>, F> {
    cursor: C,
    stopped: bool,
    only_first: bool,
    f: F,
    _phantom: PhantomData<T>,
}

impl<T: Clone + Debug, C: Cursor<T>, F> Consume<T, C, F> {
    fn new(cursor: C, f: F, only_first: bool) -> Self {
        Self {
            cursor,
            f,
            stopped: false,
            only_first,
            _phantom: PhantomData,
        }
    }
}

impl<T: Clone + Debug, C: Cursor<T>, F> Debug for Consume<T, C, F>
where
    T: Debug + Clone,
    C: Cursor<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Consume")
            .field("cursor", &self.cursor)
            .field("only_first", &self.only_first)
            .field("stopped", &self.stopped)
            .finish()
    }
}

impl<T, C, F> Cursor<T> for Consume<T, C, F>
where
    T: Clone + Debug,
    C: Cursor<T>,
    F: FnMut(C::Item) -> bool + Clone,
{
    type Item = C::Item;

    fn bump(&mut self) -> Option<Self::Item> {
        let next = self.next();
        let f = &mut self.f;

        if self.stopped {
            None
        } else if f(next.clone()) && self.only_first {
            self.stopped = true;
            self.bump()
        } else if f(next) {
            self.bump()
        } else {
            self.stopped = true;
            None
        }
    }

    fn curr(&self) -> Self::Item {
        self.cursor.curr()
    }

    fn start(&self) -> usize {
        self.cursor.start()
    }

    fn end(&self) -> usize {
        self.cursor.end()
    }
}

impl<T: Clone + Debug, C: Cursor<T>, F: FnMut(C::Item) -> bool> AttemptNext<T, C, F> for C {
    fn attempt_next(self, mut predicate: F) -> Option<C> {
        if predicate(self.next()) {
            Some(self)
        } else {
            None
        }
    }
}

impl<R: FromCursor<T, C::Item>, T: Clone + Debug, C: Cursor<T>, F: FnMut(R) -> Option<T>>
    ToToken<R, T, C, F> for C
{
    fn to_token(self, mut predicate: F) -> Option<SpanWrapper<T>> {
        let span = Span::new(self.start(), self.end());
        let val = predicate(self.collect())?;

        let token = SpanWrapper::new(val, span);
        Some(token)
    }
}

impl<T: Clone + Debug, C: Cursor<T>, F: FnMut(C::Item) -> bool> ConsumeNext<T, C, F> for C {
    type Output = Consume<T, C, F>;

    fn consume_next(self, predicate: F) -> Self::Output {
        Consume::new(self, predicate, true)
    }
}

impl<T: Clone + Debug, C: Cursor<T>, F: FnMut(C::Item) -> bool> ConsumeWhile<T, C, F> for C {
    type Output = Consume<T, C, F>;

    fn consume_while(self, predicate: F) -> Self::Output {
        Consume::new(self, predicate, false)
    }
}

impl<T: Clone + Debug, C: Cursor<T>, F: FnMut(C::Item) -> bool> AttemptNext<T, C, F> for Option<C> {
    fn attempt_next(self, predicate: F) -> Option<C> {
        self.map_or(None, |c| c.attempt_next(predicate))
    }
}

impl<R: FromCursor<T, C::Item>, T: Clone + Debug, C: Cursor<T>, F: FnMut(R) -> Option<T>>
    ToToken<R, T, C, F> for Option<C>
{
    fn to_token(self, predicate: F) -> Option<SpanWrapper<T>> {
        self.map_or(None, |c| c.to_token(predicate))
    }
}

impl<T: Clone + Debug, C: Cursor<T>, F: FnMut(C::Item) -> bool> ConsumeNext<T, C, F> for Option<C> {
    type Output = Option<Consume<T, C, F>>;

    fn consume_next(self, predicate: F) -> Self::Output {
        self.map(|c| Consume::new(c, predicate, true))
    }
}

impl<T: Clone + Debug, C: Cursor<T>, F: FnMut(C::Item) -> bool> ConsumeWhile<T, C, F>
    for Option<C>
{
    type Output = Option<Consume<T, C, F>>;

    fn consume_while(self, predicate: F) -> Self::Output {
        self.map(|c| Consume::new(c, predicate, false))
    }
}
