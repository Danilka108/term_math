use super::super::cursor::{Cursor, OriginalItem};
use super::look::Look;
use super::Modification;
use crate::span::SpanWrapper;
use std::fmt::Debug;

pub trait Compare<FN, ERR: Clone + Debug> {
    type Res: Clone + Debug;

    fn expect(self, predicate: FN) -> Modification<Self::Res, SpanWrapper<ERR>>;

    fn skip_if(self, predicate: FN) -> Modification<Self::Res, SpanWrapper<ERR>>;

    fn require(self, predicate: FN, err: ERR) -> Modification<Self::Res, SpanWrapper<ERR>>;

    fn throw_if(self, predicate: FN, err: ERR) -> Modification<Self::Res, SpanWrapper<ERR>>;
}

impl<CURSOR: Cursor, FN: FnMut(OriginalItem<CURSOR>) -> bool, ERR: Clone + Debug> Compare<FN, ERR>
    for Look<CURSOR>
{
    type Res = Look<CURSOR>;

    fn expect(self, mut predicate: FN) -> Modification<Self::Res, SpanWrapper<ERR>> {
        if predicate(self.look().val()) {
            Modification::Ok(self)
        } else {
            Modification::None
        }
    }

    fn skip_if(self, mut predicate: FN) -> Modification<Self::Res, SpanWrapper<ERR>> {
        if predicate(self.look().val()) {
            Modification::None
        } else {
            Modification::Ok(self)
        }
    }

    fn require(self, mut predicate: FN, err: ERR) -> Modification<Self::Res, SpanWrapper<ERR>> {
        if predicate(self.look().val()) {
            Modification::Ok(self)
        } else {
            Modification::Err(SpanWrapper::new(err, self.span()))
        }
    }

    fn throw_if(self, mut predicate: FN, err: ERR) -> Modification<Self::Res, SpanWrapper<ERR>> {
        if predicate(self.look().val()) {
            Modification::Err(SpanWrapper::new(err, self.span()))
        } else {
            Modification::Ok(self)
        }
    }
}

impl<CURSOR: Cursor, FN: FnMut(OriginalItem<CURSOR>) -> bool, ERR: Clone + Debug> Compare<FN, ERR>
    for Modification<Look<CURSOR>, SpanWrapper<ERR>>
{
    type Res = Look<CURSOR>;

    fn expect(self, predicate: FN) -> Modification<Self::Res, SpanWrapper<ERR>> {
        match self {
            Modification::Ok(next) => next.expect(predicate),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }

    fn skip_if(self, predicate: FN) -> Modification<Self::Res, SpanWrapper<ERR>> {
        match self {
            Modification::Ok(next) => next.skip_if(predicate),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }

    fn require(self, predicate: FN, err: ERR) -> Modification<Self::Res, SpanWrapper<ERR>> {
        match self {
            Modification::Ok(next) => next.require(predicate, err),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }

    fn throw_if(self, predicate: FN, err: ERR) -> Modification<Self::Res, SpanWrapper<ERR>> {
        match self {
            Modification::Ok(next) => next.throw_if(predicate, err),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }
}
