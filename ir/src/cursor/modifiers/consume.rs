use super::super::cursor::Cursor;
use super::Modification;
use crate::span::SpanWrapper;
use std::fmt::Debug;

pub trait ConsumeOnce {
    type Res;

    fn consume_once(self) -> Self::Res;
}

pub trait ConsumeWhile<FN> {
    type Res;

    fn consume_while(self, predicate: FN) -> Self::Res;
}

pub trait ConsumeTo<FN, ERR: Clone + Debug> {
    type Consumed: Clone + Debug;

    fn consume_to(
        self,
        predicate: FN,
        skip: usize,
        err: ERR,
    ) -> Modification<Self::Consumed, SpanWrapper<ERR>>;
}

#[derive(Clone)]
pub struct Consume<CURSOR: Cursor, FN> {
    cursor: CURSOR,
    stopped: bool,
    neg: bool,
    only_first: bool,
    skip: usize,
    f: FN,
}

impl<CURSOR: Cursor, FN> Consume<CURSOR, FN> {
    fn new_while(cursor: CURSOR, f: FN) -> Self {
        Self {
            cursor,
            f,
            stopped: false,
            only_first: false,
            neg: false,
            skip: 0,
        }
    }

    fn new_once(cursor: CURSOR, f: FN) -> Self {
        Self {
            cursor,
            f,
            stopped: false,
            only_first: true,
            neg: false,
            skip: 0,
        }
    }

    pub fn new_to(cursor: CURSOR, f: FN, skip: usize) -> Self {
        Self {
            cursor,
            f,
            stopped: false,
            only_first: false,
            neg: true,
            skip,
        }
    }
}

impl<CURSOR: Cursor, FN> Debug for Consume<CURSOR, FN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Consume")
            .field("cursor", &self.cursor)
            .field("only_first", &self.only_first)
            .field("stopped", &self.stopped)
            .field("neg", &self.neg)
            .field("skip", &self.skip)
            .finish()
    }
}

impl<CURSOR: Cursor, FN: Clone + FnMut(CURSOR::Item) -> bool> Cursor for Consume<CURSOR, FN> {
    type Item = CURSOR::Item;
    type Original = CURSOR::Original;

    fn bump(&mut self) -> Option<SpanWrapper<Self::Item>> {
        if self.stopped || self.cursor.is_eof() {
            return None;
        }

        let next = self.cursor.next().val();

        if (&mut self.f)(next.clone()) && self.skip > 0 {
            self.skip -= 1;
            return self.cursor.bump();
        }

        let mut f = |item: Self::Item| {
            if self.neg {
                !(&mut self.f)(item)
            } else {
                (&mut self.f)(item)
            }
        };

        if f(next.clone()) && self.only_first {
            self.stopped = true;
            return self.cursor.bump();
        }

        if f(next) {
            return self.cursor.bump();
        }

        self.stopped = true;
        None
    }

    fn curr(&self) -> SpanWrapper<Self::Item> {
        self.cursor.curr()
    }

    fn original(&self) -> &Self::Original {
        self.cursor.original()
    }
}

impl<CURSOR: Cursor> ConsumeOnce for CURSOR {
    type Res = Consume<CURSOR, fn(CURSOR::Item) -> bool>;

    fn consume_once(self) -> Self::Res {
        #[inline]
        fn call<CURSOR: Cursor>(_: CURSOR::Item) -> bool {
            true
        }

        Consume::new_once(self, call::<CURSOR>)
    }
}

impl<CURSOR: Cursor, FN: FnMut(CURSOR::Item) -> bool> ConsumeWhile<FN> for CURSOR {
    type Res = Consume<CURSOR, FN>;

    fn consume_while(self, predicate: FN) -> Self::Res {
        Consume::new_while(self, predicate)
    }
}

impl<CURSOR: Cursor, FN: Clone + FnMut(CURSOR::Item) -> bool, ERR: Clone + Debug> ConsumeTo<FN, ERR>
    for CURSOR
{
    type Consumed = Consume<CURSOR, FN>;

    fn consume_to(
        self,
        mut predicate: FN,
        skip: usize,
        err: ERR,
    ) -> Modification<Self::Consumed, SpanWrapper<ERR>> {
        let mut cloned = self.clone();

        let mut count = skip;
        let was_found = loop {
            if cloned.is_eof() {
                break false;
            }

            if predicate(cloned.next().val()) && count > 0 {
                count = count.checked_sub(1).unwrap_or(0);
                cloned.bump();
                continue;
            }

            if predicate(cloned.next().val()) {
                break true;
            }

            cloned.bump();
        };

        if was_found {
            Modification::Ok(Consume::new_to(self, predicate, skip))
        } else {
            Modification::Err(SpanWrapper::new(err, self.span()))
        }
    }
}

impl<CURSOR: Cursor, ERR: Clone + Debug> ConsumeOnce for Modification<CURSOR, SpanWrapper<ERR>> {
    type Res = Modification<Consume<CURSOR, fn(CURSOR::Item) -> bool>, SpanWrapper<ERR>>;

    fn consume_once(self) -> Self::Res {
        match self {
            Modification::Ok(cursor) => Modification::Ok(cursor.consume_once()),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }
}

impl<CURSOR: Cursor, FN: Clone + FnMut(CURSOR::Item) -> bool, ERR: Clone + Debug> ConsumeWhile<FN>
    for Modification<CURSOR, ERR>
{
    type Res = Modification<Consume<CURSOR, FN>, ERR>;

    fn consume_while(self, predicate: FN) -> Self::Res {
        match self {
            Modification::Ok(cursor) => Modification::Ok(cursor.consume_while(predicate)),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }
}

impl<CURSOR: Cursor, FN: Clone + FnMut(CURSOR::Item) -> bool, ERR: Clone + Debug> ConsumeTo<FN, ERR>
    for Modification<CURSOR, SpanWrapper<ERR>>
{
    type Consumed = Consume<CURSOR, FN>;

    fn consume_to(
        self,
        predicate: FN,
        skip: usize,
        err: ERR,
    ) -> Modification<Self::Consumed, SpanWrapper<ERR>> {
        match self {
            Modification::Ok(cursor) => cursor.consume_to(predicate, skip, err),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }
}
