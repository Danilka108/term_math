use super::constants::{DECIMAL_RADIX_I16, ZERO_AS_U8};
use super::digit_stack::{GetDigit, PushDigit};
use super::RealNumber;
use crate::sign::Sign;
use std::ops::Sub;

impl RealNumber {
    fn subtract_unsigned_int_parts(
        reduced_part: &Vec<char>,
        subtracted_part: &Vec<char>,
    ) -> Vec<char> {
        let delta_len = reduced_part.len() - subtracted_part.len();
        let mut buffer = 0;
        let mut new_part = Vec::new();

        for i in (0..reduced_part.len()).rev() {
            let subtract = buffer + reduced_part.get_digit_i16(i).unwrap_or(0)
                - subtracted_part.get_digit_i16(i - delta_len).unwrap_or(0);

            let subtract = if subtract < 0 {
                buffer = -1;
                subtract + DECIMAL_RADIX_I16
            } else {
                buffer = 0;
                subtract
            };

            new_part.push_digit_i16(subtract % DECIMAL_RADIX_I16);
        }

        new_part.reverse();
        new_part
    }

    fn subtract_unsigned_frac_parts(
        reduced_part: &Vec<char>,
        subtracted_part: &Vec<char>,
    ) -> (Vec<char>, Vec<char>) {
        let max_len = reduced_part.len().max(subtracted_part.len());

        let mut buffer = 0;
        let mut new_part = Vec::new();

        for i in (0..max_len).rev() {
            let subtract = reduced_part.get_digit_i16(i).unwrap_or(0) + buffer
                - subtracted_part.get_digit_i16(i).unwrap_or(0);

            let subtract = if subtract < 0 {
                buffer = -1;
                subtract + DECIMAL_RADIX_I16
            } else {
                buffer = 0;
                subtract
            };

            new_part.push_digit_i16(subtract);
        }

        new_part.reverse();
        (vec![(buffer as u8 + ZERO_AS_U8) as char], new_part)
    }

    pub(super) fn subtract_unsigned(reduced_num: &Self, subtracted_num: &Self) -> Self {
        let (subtractional_int_part, new_frac_part) = Self::subtract_unsigned_frac_parts(
            &reduced_num.fractional_part,
            &subtracted_num.fractional_part,
        );
        let new_int_part = Self::subtract_unsigned_int_parts(
            &Self::subtract_unsigned_int_parts(&reduced_num.integer_part, &subtractional_int_part),
            &subtracted_num.integer_part,
        );

        Self::new(new_int_part, new_frac_part, Sign::Positive)
    }
}

impl Sub for RealNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (max, min) = if self < other {
            (-other, self)
        } else {
            (self, -other)
        };

        if max.sign == min.sign {
            Self::add_unsigned(&max, &min)
        } else {
            Self::subtract_unsigned(&max, &min)
        }
        .set_sign(max.sign)
    }
}
