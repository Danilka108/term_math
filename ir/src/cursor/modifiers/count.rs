use super::super::cursor::Cursor;
use super::modification::Modification;
use crate::span::SpanWrapper;
use std::fmt::Debug;

pub trait Count<O, S> {
    type Res;

    fn count_to(self, is_occurrence: O, is_stop: S) -> Self::Res;
}

impl<CURSOR: Cursor, O: FnMut(CURSOR::Item) -> bool, S: FnMut(CURSOR::Item) -> bool> Count<O, S> for CURSOR {
    type Res = usize;

    fn count_to(mut self, mut is_occurrence: O, mut is_stop: S) -> Self::Res {
        let mut count = 0;

        loop {
            if self.is_eof() || is_stop(self.next().val()) {
                break;
            }

            if is_occurrence(self.next().val()) {
                count += 1;
            }

            self.bump();
        } 
       
        count
    }
}

impl<CURSOR: Cursor, O: FnMut(CURSOR::Item) -> bool, S: FnMut(CURSOR::Item) -> bool, ERR: Clone + Debug> Count<O, S> for Modification<CURSOR, SpanWrapper<ERR>> {
    type Res = Modification<usize, SpanWrapper<ERR>>;
    
    fn count_to(self, is_occurrence: O, is_stop: S) -> Self::Res {
        match self {
            Modification::Ok(cursor) => Modification::Ok(cursor.count_to(is_occurrence, is_stop)),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }
}
