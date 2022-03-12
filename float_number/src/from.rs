use crate::{FloatNumber, Sign};

impl FloatNumber {
    pub fn from_unsigned_numeric_string(unsigned_numeric_string: String) -> Option<Self> {
        let chars = unsigned_numeric_string.chars().collect::<Vec<_>>();
        let mut parts = chars.split(|&sym| sym == '.');

        let int_part = parts.next();
        let frac_part = parts.next();

        if parts.next().is_some() {
            return None;
        }

        let (exponent, significand) = match (int_part, frac_part) {
            (Some(int_part), Some(frac_part)) if int_part.len() != 0 && frac_part.len() != 0 => {
                let exponent = -(frac_part.len() as isize);
                let significand = [int_part, frac_part].concat();
                (exponent, significand)
            }
            (Some(int_part), None) if int_part.len() != 0 => (0, int_part.to_vec()),
            _ => return None,
        };

        let are_digits_valid = significand
            .iter()
            .fold(true, |acc, &sym| acc && sym.is_digit(Self::RADIX));

        if !are_digits_valid {
            return None;
        }

        let significand = significand
            .iter()
            .map(|&sym| sym.to_digit(Self::RADIX).unwrap())
            .collect();

        Some(Self::new(significand, exponent, Sign::Pos))
    }
}
