use super::constants::{DECIMAL_RADIX, ZERO_AS_U8};
use super::digit_stack::{GetDigit, PushDigit};
use super::RealNumber;
use crate::sign::Sign;
use std::ops::Add;

impl RealNumber {
    fn add_unsigned_int_parts(a_part: &Vec<char>, b_part: &Vec<char>) -> Vec<char> {
        let (max_part, min_part) = if a_part.len() < b_part.len() {
            (b_part, a_part)
        } else {
            (a_part, b_part)
        };
        let delta_len = max_part.len() - min_part.len();

        let mut buffer = 0;
        let mut new_part = Vec::new();

        for i in (0..max_part.len()).rev() {
            let sum = buffer
                + max_part.get_digit(i).unwrap_or(0)
                + min_part.get_digit(i - delta_len).unwrap_or(0);

            new_part.push_digit(sum % DECIMAL_RADIX);
            buffer = sum / DECIMAL_RADIX;
        }

        if buffer != 0 {
            new_part.push_digit(buffer);
        }

        new_part.reverse();
        new_part
    }

    fn add_unsigned_frac_parts(a_part: &Vec<char>, b_part: &Vec<char>) -> (Vec<char>, Vec<char>) {
        let (max_part, min_part) = if a_part.len() < a_part.len() {
            (b_part, a_part)
        } else {
            (a_part, b_part)
        };

        let mut buffer = 0;
        let mut new_part = Vec::new();

        for i in (0..max_part.len()).rev() {
            let sum =
                buffer + max_part.get_digit(i).unwrap_or(0) + min_part.get_digit(i).unwrap_or(0);
            new_part.push_digit(sum % DECIMAL_RADIX);
            buffer = sum / DECIMAL_RADIX;
        }

        new_part.reverse();
        (vec![(buffer + ZERO_AS_U8) as char], new_part)
    }

    pub(super) fn add_unsigned(a_num: &Self, b_num: &Self) -> Self {
        let (additional_int_part, new_frac_part) =
            Self::add_unsigned_frac_parts(&a_num.fractional_part, &b_num.fractional_part);
        let new_int_part = Self::add_unsigned_int_parts(
            &Self::add_unsigned_int_parts(&a_num.integer_part, &additional_int_part),
            &b_num.integer_part,
        );

        RealNumber::new(new_int_part, new_frac_part, Sign::Positive)
    }
}

impl Add for RealNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (max, min) = if self < other {
            (other, self)
        } else {
            (self, other)
        };

        if max.sign == min.sign {
            Self::add_unsigned(&max, &min)
        } else {
            Self::subtract_unsigned(&max, &min)
        }
        .set_sign(max.sign)
    }
}
