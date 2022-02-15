use super::RealNumber;
use crate::sign::Sign;

impl PartialEq for RealNumber {
    fn eq(&self, other: &Self) -> bool {
        let parts = [
            (self.integer_part.clone(), other.integer_part.clone()),
            (self.fractional_part.clone(), other.fractional_part.clone()),
        ];

        for (self_part, other_part) in parts {
            let zipped_parts = self_part.iter().zip(other_part.iter());

            for (self_chr, other_chr) in zipped_parts {
                if self_chr != other_chr {
                    return false;
                }
            }
        }

        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => true,
            _ => false,
        }
    }
}

impl Eq for RealNumber {}
