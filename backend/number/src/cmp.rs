use crate::number::{Number, NumberKind};
use std::cmp::Ordering;

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => false,
            (NumberKind::Real(s), NumberKind::Real(o)) => s.eq(o),
        }
    }

    fn ne(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => false,
            (NumberKind::Real(s), NumberKind::Real(o)) => s.ne(o),
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.kind, &other.kind) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => None,
            (NumberKind::Real(s), NumberKind::Real(o)) => s.partial_cmp(o),
        }
    }
}
