use super::super::cursor::{Cursor, FromCursor};
use super::Modification;
use crate::span::SpanWrapper;
use std::fmt::Debug;

pub trait Produce<CURSOR, FROM, PRODUCT, ERR, FN>
where
    CURSOR: Cursor,
    FROM: FromCursor<CURSOR::Item>,
    PRODUCT: Clone + Debug,
    ERR: Clone + Debug,
    FN: FnMut(FROM) -> Modification<PRODUCT, ERR>,
{
    fn produce(self, predicate: FN) -> Modification<SpanWrapper<PRODUCT>, SpanWrapper<ERR>>;

    fn produce_not_empty(self, predicate: FN, if_empty: ERR) -> Modification<SpanWrapper<PRODUCT>, SpanWrapper<ERR>>;
}

impl<CURSOR, FROM, PRODUCT, ERR, FN> Produce<CURSOR, FROM, PRODUCT, ERR, FN> for CURSOR
where
    CURSOR: Cursor,
    FROM: FromCursor<CURSOR::Item>,
    PRODUCT: Clone + Debug,
    ERR: Clone + Debug,
    FN: FnMut(FROM) -> Modification<PRODUCT, ERR>,
{
    fn produce(
        self,
        mut predicate: FN,
    ) -> Modification<SpanWrapper<PRODUCT>, SpanWrapper<ERR>> {
        let span = self.span();

        if self.is_eof() {
            return Modification::None;
        }

        let val = match predicate(self.collect()) {
            Modification::Ok(val) => val,
            Modification::Err(err) => return Modification::Err(SpanWrapper::new(err, span)),
            Modification::None => return Modification::None,
        };
        let token = SpanWrapper::new(val, span);

        Modification::Ok(token)
    }

    fn produce_not_empty(
        self,
        mut predicate: FN,
        if_empty: ERR,
    ) -> Modification<SpanWrapper<PRODUCT>, SpanWrapper<ERR>> {
        let span = self.span();

        if self.is_eof() {
            return Modification::Err(SpanWrapper::new(if_empty, span));
        }

        let val = match predicate(self.collect()) {
            Modification::Ok(val) => val,
            Modification::Err(err) => return Modification::Err(SpanWrapper::new(err, span)),
            Modification::None => return Modification::None,
        };
        let token = SpanWrapper::new(val, span);

        Modification::Ok(token)
    }
}

impl<CURSOR, FROM, PRODUCT, ERR, FN> Produce<CURSOR, FROM, PRODUCT, ERR, FN> for Modification<CURSOR, SpanWrapper<ERR>>
where
    CURSOR: Cursor,
    FROM: FromCursor<CURSOR::Item>,
    PRODUCT: Clone + Debug,
    ERR: Clone + Debug,
    FN: FnMut(FROM) -> Modification<PRODUCT, ERR>,
{
    fn produce(
        self,
        predicate: FN,
    ) -> Modification<SpanWrapper<PRODUCT>, SpanWrapper<ERR>> {
        match self {
            Modification::Ok(cursor) => cursor.produce(predicate),
            Modification::Err(err) => return Modification::Err(err),
            Modification::None => return Modification::None,
        }
    }

    fn produce_not_empty(
        self,
        predicate: FN,
        if_empty: ERR,
    ) -> Modification<SpanWrapper<PRODUCT>, SpanWrapper<ERR>> {
        match self {
            Modification::Ok(cursor) => cursor.produce_not_empty(predicate, if_empty),
            Modification::Err(err) => return Modification::Err(err),
            Modification::None => return Modification::None,
        }
    }
}
