use crate::{FloatNumber, Sign};
use std::cmp::Ordering;

impl FloatNumber {
    fn ucmp(&self, other: &Self) -> Ordering {
        match self.int_part_len().cmp(&other.int_part_len()) {
            Ordering::Equal => (),
            ord => return ord,
        }

        let digits = self.significand.iter().zip(other.significand.iter());

        for (self_digit, other_digit) in digits {
            match self_digit.cmp(other_digit) {
                Ordering::Equal => continue,
                ord => return ord,
            }
        }

        Ordering::Equal
    }

    pub(crate) fn cmp(&self, other: &Self) -> Ordering {
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }

        match (&self.sign, &other.sign) {
            (Sign::Pos, Sign::Pos) => self.ucmp(other),
            (Sign::Neg, Sign::Neg) => self.ucmp(other).reverse(),
            (Sign::Pos, Sign::Neg) => Ordering::Greater,
            (Sign::Neg, Sign::Pos) => Ordering::Less,
        }
    }

    pub(crate) fn is_umax(&self, other: &Self) -> bool {
        match self.ucmp(other) {
            Ordering::Greater | Ordering::Equal => true,
            Ordering::Less => false,
        }
    }

    pub(crate) fn is_umin(&self, other: &Self) -> bool {
        match self.ucmp(other) {
            Ordering::Less => true,
            Ordering::Greater | Ordering::Equal => false,
        }
    }

    pub(crate) fn get_umax_umin(self, other: Self) -> (Self, Self) {
        if self.is_umax(&other) {
            (self, other)
        } else {
            (other, self)
        }
    }

    pub(crate) fn is_ueq(&self, other: &Self) -> bool {
        match self.ucmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }

    pub(crate) fn is_eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}
