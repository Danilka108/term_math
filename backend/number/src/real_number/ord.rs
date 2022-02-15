use super::RealNumber;
use std::cmp::Ordering;
use crate::sign::Sign;

impl PartialOrd for RealNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RealNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        let match_chrs = |self_chr, other_chr| match (self.sign, other.sign) {
            (Sign::Negative, Sign::Positive) if self_chr != other_chr => Some(Ordering::Less),
            (Sign::Positive, Sign::Negative) if self_chr != other_chr => Some(Ordering::Greater),

            (Sign::Positive, Sign::Positive) if self_chr > other_chr => Some(Ordering::Greater),
            (Sign::Positive, Sign::Positive) if self_chr < other_chr => Some(Ordering::Less),
            (Sign::Negative, Sign::Negative) if self_chr > other_chr => Some(Ordering::Less),
            (Sign::Negative, Sign::Negative) if self_chr < other_chr => Some(Ordering::Greater),

            _ => None,
        };

        let parts = [
            (self.integer_part.clone(), other.integer_part.clone()),
            (self.fractional_part.clone(), other.fractional_part.clone()),
        ];

        for (self_part, other_part) in parts {
            let zipped_parts = self_part.iter().zip(other_part.iter());

            for (self_chr, other_chr) in zipped_parts {
                if let Some(ord) = match_chrs(self_chr.clone(), other_chr.clone()) {
                    return ord;
                }
            }
        }

        match (self.sign, other.sign) {
            (Sign::Negative, Sign::Positive) => Ordering::Less,
            (Sign::Positive, Sign::Negative) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}
