use crate::FloatNumber;
use std::cmp::Ordering;

impl FloatNumber {
    pub(crate) fn ucmp(&self, other: &Self) -> Ordering {
        match self.int_part_len().cmp(&other.int_part_len()) {
            Ordering::Equal => (),
            ord => return ord,
        }

        let start_bound = self.start_bound().min(other.start_bound());
        let end_bound = self.end_bound().max(other.end_bound());

        for pos in (start_bound..end_bound).rev() {
            let (self_digit, other_digit) = match (self.get_digit(pos), other.get_digit(pos)) {
                (Some(s), Some(o)) => (s, o),
                _ => break,
            };

            match self_digit.cmp(&other_digit) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }

        Ordering::Equal
    }

    pub(crate) fn icmp(&self, other: &Self) -> Ordering {
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }

        match self.cmp_sign(other) {
            Ordering::Equal if self.is_neg() => self.ucmp(other).reverse(),
            Ordering::Equal => self.ucmp(other),
            ord => ord,
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

    pub(crate) fn is_ieq(&self, other: &Self) -> bool {
        match self.icmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}
