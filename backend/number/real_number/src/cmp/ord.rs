use crate::sign::Sign;
use crate::RealNumber;
use std::cmp::Ordering;

impl RealNumber {
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
        let a_num = self;
        let b_num = other;

        let sign = match (a_num.sign, b_num.sign) {
            (Sign::Negative, Sign::Positive) => return Ordering::Less,
            (Sign::Positive, Sign::Negative) => return Ordering::Greater,
            (sign, _) => sign,
        };

        let match_chrs = |a_chr, b_chr| match sign {
            Sign::Positive if a_chr > b_chr => Some(Ordering::Greater),
            Sign::Positive if a_chr < b_chr => Some(Ordering::Less),
            Sign::Negative if a_chr > b_chr => Some(Ordering::Less),
            Sign::Negative if a_chr < b_chr => Some(Ordering::Greater),
            _ => None,
        };

        match a_num.int_len().cmp(&b_num.int_len()) {
            Ordering::Less => match sign {
                Sign::Positive => return Ordering::Less,
                Sign::Negative => return Ordering::Greater,
            },
            Ordering::Greater => match sign {
                Sign::Positive => return Ordering::Greater,
                Sign::Negative => return Ordering::Less,
            },
            Ordering::Equal => (),
        }

        for (&a_chr, b_chr) in [a_num.integer_part.clone(), a_num.fractional_part.clone()]
            .concat()
            .iter()
            .zip([b_num.integer_part.clone(), b_num.fractional_part.clone()].concat())
        {
            if let Some(ord) = match_chrs(a_chr, b_chr) {
                return ord;
            }
        }

        match a_num.frac_len().cmp(&b_num.frac_len()) {
            Ordering::Less => match sign {
                Sign::Positive => Ordering::Less,
                Sign::Negative => Ordering::Greater,
            },
            Ordering::Greater => match sign {
                Sign::Positive => Ordering::Greater,
                Sign::Negative => Ordering::Less,
            },
            Ordering::Equal => Ordering::Equal,
        }
    }
}
