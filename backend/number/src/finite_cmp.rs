use crate::finite::Finite;
use std::cmp::Ordering;

pub fn unsigned_finite_cmp<const RADIX: u32, const PREC: isize>(
    lhs: &Finite<RADIX, PREC>,
    rhs: &Finite<RADIX, PREC>,
) -> Ordering {
    match lhs.int_len().cmp(&rhs.int_len()) {
        Ordering::Equal => (),
        ord => return ord,
    }

    for pos in lhs.merge_bounds(&rhs).rev() {
        let lhs_digit = lhs.get_digit(pos).unwrap_or(0);
        let rhs_digit = rhs.get_digit(pos).unwrap_or(0);

        match lhs_digit.cmp(&rhs_digit) {
            Ordering::Equal => (),
            ord => return ord,
        }
    }

    Ordering::Equal
}

pub fn is_unsigned_finite_eq<const RADIX: u32, const PREC: isize>(
    lhs: &Finite<RADIX, PREC>,
    rhs: &Finite<RADIX, PREC>,
) -> bool {
    matches!(unsigned_finite_cmp(&lhs, &rhs), Ordering::Equal)
}

pub fn get_unsigned_finite_max_min<const RADIX: u32, const PREC: isize>(
    lhs: Finite<RADIX, PREC>,
    rhs: Finite<RADIX, PREC>,
) -> (Finite<RADIX, PREC>, Finite<RADIX, PREC>) {
    match unsigned_finite_cmp(&lhs, &rhs) {
        Ordering::Less => (rhs, lhs),
        Ordering::Greater | Ordering::Equal => (lhs, rhs),
    }
}

pub fn finite_cmp<const RADIX: u32, const PREC: isize>(
    lhs: &Finite<RADIX, PREC>,
    rhs: &Finite<RADIX, PREC>,
) -> Ordering {
    if lhs.is_zero() && rhs.is_zero() {
        return Ordering::Equal;
    }

    match lhs.cmp_sign(&rhs) {
        Ordering::Equal if lhs.is_neg() => unsigned_finite_cmp(lhs,rhs).reverse(),
        Ordering::Equal => unsigned_finite_cmp(lhs, rhs),
        ord => ord,
    }
}
