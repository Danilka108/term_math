use crate::sign::Sign;
use crate::RealNumber;
use std::cmp::Ordering;

impl RealNumber {
    fn is_zero(&self) -> bool {
        match (self.integer_part.first(), self.fractional_part.first()) {
            (Some('0'), Some('0')) if self.int_len() == 1 && self.frac_len() == 1 => true,
            _ => false,
        }
    }

    pub(crate) fn umax(self, other: Self) -> Self {
        let self_sign = self.sign;
        let other_sign = other.sign;

        if self.clone().set_sign(Sign::Positive) > other.clone().set_sign(Sign::Positive) {
            self.set_sign(self_sign)
        } else {
            other.set_sign(other_sign)
        }
    }

    pub(crate) fn umin(self, other: Self) -> Self {
        let self_sign = self.sign;
        let other_sign = other.sign;

        if self.clone().set_sign(Sign::Positive) <= other.clone().set_sign(Sign::Positive) {
            self.set_sign(self_sign)
        } else {
            other.set_sign(other_sign)
        }
    }
}

impl PartialOrd for RealNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<usize> for RealNumber {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        Some(self.cmp(&Self::from_usize(*other)))
    }
}

impl Ord for RealNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        let a_num = if self.is_zero() {
            Self::zero()
        } else {
            self.clone()
        };

        let b_num = if other.is_zero() {
            Self::zero()
        } else {
            other.clone()
        };

        let sign = match (a_num.sign, b_num.sign) {
            (Sign::Negative, Sign::Positive) => return Ordering::Less,
            (Sign::Positive, Sign::Negative) => return Ordering::Greater,
            (sign, _) => sign,
        };

        match a_num.len().cmp(&b_num.len()) {
            Ordering::Equal => (),
            Ordering::Less => match sign {
                Sign::Positive => return Ordering::Less,
                Sign::Negative => return Ordering::Greater,
            },
            Ordering::Greater => match sign {
                Sign::Positive => return Ordering::Greater,
                Sign::Negative => return Ordering::Less,
            },
        }

        let match_chrs = |a_chr, b_chr| match sign {
            Sign::Positive if a_chr > b_chr => Some(Ordering::Greater),
            Sign::Positive if a_chr < b_chr => Some(Ordering::Less),
            Sign::Negative if a_chr > b_chr => Some(Ordering::Less),
            Sign::Negative if a_chr < b_chr => Some(Ordering::Greater),
            _ => None,
        };

        for (&a_chr, b_chr) in [a_num.integer_part.clone(), a_num.fractional_part.clone()]
            .concat()
            .iter()
            .zip([b_num.integer_part.clone(), b_num.fractional_part.clone()].concat())
        {
            if let Some(ord) = match_chrs(a_chr, b_chr) {
                return ord;
            }
        }

        Ordering::Equal
    }
}
