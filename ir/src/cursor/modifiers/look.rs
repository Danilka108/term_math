use super::super::cursor::{Cursor, OriginalItem};
use super::Modification;
use crate::span::SpanWrapper;
use std::fmt::Debug;

pub trait LookAt {
    type Res;

    fn look_at_curr(self) -> Self::Res;

    fn look_at_first(self) -> Self::Res;

    fn look_at_second(self) -> Self::Res;
}

#[derive(Clone, Debug)]
pub struct Look<CURSOR: Cursor> {
    cursor: CURSOR,
    offset: usize,
}

impl<CURSOR: Cursor> Cursor for Look<CURSOR> {
    type Item = CURSOR::Item;
    type Original = CURSOR::Original;

    fn bump(&mut self) -> Option<SpanWrapper<Self::Item>> {
        self.cursor.bump()
    }

    fn curr(&self) -> SpanWrapper<Self::Item> {
        self.cursor.curr()
    }

    fn original(&self) -> &Self::Original {
        self.cursor.original()
    }
}

impl<CURSOR: Cursor> Look<CURSOR> {
    fn new(cursor: CURSOR, offset: usize) -> Self {
        Self { cursor, offset }
    }

    pub(super) fn look(&self) -> SpanWrapper<OriginalItem<CURSOR>> {
        let mut orig = self.original().clone();

        for _ in 0..self.offset {
            orig.bump();
        }

        orig.curr()
    }
}

impl<CURSOR: Cursor> LookAt for CURSOR {
    type Res = Look<CURSOR>;

    fn look_at_curr(self) -> Self::Res {
        Look::new(self, 0)
    }

    fn look_at_first(self) -> Self::Res {
        Look::new(self, 1)
    }

    fn look_at_second(self) -> Self::Res {
        Look::new(self, 2)
    }
}

impl<CURSOR: Cursor, ERR: Clone + Debug> LookAt for Modification<CURSOR, SpanWrapper<ERR>> {
    type Res = Modification<Look<CURSOR>, SpanWrapper<ERR>>;

    fn look_at_curr(self) -> Self::Res {
        match self {
            Modification::Ok(cursor) => Modification::Ok(Look::new(cursor, 0)),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }

    fn look_at_first(self) -> Self::Res {
        match self {
            Modification::Ok(cursor) => Modification::Ok(Look::new(cursor, 1)),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }

    fn look_at_second(self) -> Self::Res {
        match self {
            Modification::Ok(cursor) => Modification::Ok(Look::new(cursor, 2)),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }
}
