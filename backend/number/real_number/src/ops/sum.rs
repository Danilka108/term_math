use constants::{DECIMAL_RADIX, DECIMAL_RADIX_I16};
use crate::sign::Sign;
use crate::vec::{GetDigit, Longest, PushDigit, Shortest};
use crate::RealNumber;

impl RealNumber {
    fn unsigned_add_int_parts(a_part: Vec<char>, b_part: Vec<char>) -> Vec<char> {
        let long_part = a_part.longest(&b_part);
        let short_part = a_part.shortest(&b_part);
        let delta_len = long_part.len() - short_part.len();

        let mut new_part = Vec::new();
        let mut buffer = 0;

        for i in (0..long_part.len()).rev() {
            let sum = buffer
                + long_part.get_digit(i).unwrap_or(0)
                + if i >= delta_len {
                    short_part.get_digit(i - delta_len).unwrap_or(0)
                } else {
                    0
                };

            new_part.push_digit(sum % DECIMAL_RADIX);
            buffer = sum / DECIMAL_RADIX;
        }

        if buffer != 0 {
            new_part.push_digit(buffer);
        }

        new_part.reverse();
        new_part
    }

    fn unsigned_add(mut a_num: Self, mut b_num: Self) -> Self {
        let offset_degree = a_num.frac_len().max(b_num.frac_len());

        a_num >>= offset_degree;
        b_num >>= offset_degree;

        let int_part = Self::unsigned_add_int_parts(a_num.integer_part, b_num.integer_part);

        Self::new(int_part, vec!['0'], Sign::Positive) << offset_degree
    }

    fn unsigned_subtract_int_parts(
        reduced_part: Vec<char>,
        subtracted_part: Vec<char>,
    ) -> Vec<char> {
        let delta_len = reduced_part.len() - subtracted_part.len();

        let mut buffer = 0;
        let mut new_part = Vec::new();

        for i in (0..reduced_part.len()).rev() {
            let subtract = buffer + reduced_part.get_digit_i16(i).unwrap_or(0)
                - if i >= delta_len {
                    subtracted_part.get_digit_i16(i - delta_len).unwrap_or(0)
                } else {
                    0
                };

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

    fn unsigned_subtract(mut reduced_num: Self, mut subtracted_num: Self) -> Self {
        let offset_degree = reduced_num.frac_len().max(subtracted_num.frac_len());

        reduced_num >>= offset_degree;
        subtracted_num >>= offset_degree;

        let int_part = Self::unsigned_subtract_int_parts(
            reduced_num.integer_part,
            subtracted_num.integer_part,
        );

        Self::new(int_part, vec!['0'], Sign::Positive) << offset_degree
    }

    pub(super) fn sum(a_num: Self, b_num: Self) -> Self {
        let max_num = a_num.clone().umax(b_num.clone());
        let min_num = a_num.umin(b_num);
        let output_sign = max_num.sign;

        if max_num.sign != min_num.sign && max_num.ueq(&min_num) {
            return Self::zero();
        }

        if max_num.sign == min_num.sign {
            Self::unsigned_add(max_num, min_num)
        } else {
            Self::unsigned_subtract(max_num, min_num)
        }
        .set_sign(output_sign)
        .trim_int_zeros()
        .trim_frac_zeros()
    }
}
