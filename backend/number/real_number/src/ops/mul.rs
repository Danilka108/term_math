use crate::constants::DECIMAL_RADIX;
use crate::sign::Sign;
use crate::vec::{GetDigit, Longest, PushDigit, Shortest};
use crate::RealNumber;
use std::ops::{Mul, MulAssign};

impl RealNumber {
    fn unsigned_mul_digit_to_int_part(int_part: &Vec<char>, digit: u8) -> Vec<char> {
        let mut new_part = Vec::new();

        let mut buffer = 0;

        for i in (0..int_part.len()).rev() {
            let mul = buffer + int_part.get_digit(i).unwrap_or(0) * digit;
            new_part.push_digit(mul % DECIMAL_RADIX);
            buffer = mul / DECIMAL_RADIX;
        }

        new_part.push_digit(buffer);
        new_part.reverse();

        new_part
    }

    fn unsigned_multiply(a_num: Self, b_num: Self) -> Self {
        let long_part = a_num.integer_part.longest(&b_num.integer_part);
        let short_part = a_num.integer_part.shortest(&b_num.integer_part);

        let mut new_num = Self::zero();

        for i in (0..short_part.len()).rev() {
            let mul_part = Self::unsigned_mul_digit_to_int_part(
                &long_part,
                short_part.get_digit(i).unwrap_or(0),
            );

            let mul_num =
                Self::new(mul_part, vec!['0'], Sign::Positive) >> (short_part.len() - (i + 1));

            new_num += mul_num;
        }

        new_num
    }

    fn multiply(a_num: Self, b_num: Self) -> Self {
        if a_num == 0 || b_num == 0 {
            return Self::zero();
        }

        if a_num == 1 {
            return b_num;
        }

        if b_num == 1 {
            return a_num;
        }

        let a_num_offset_degree = a_num.fractional_part.len();
        let b_num_offset_degree = b_num.fractional_part.len();

        let output_sign = a_num.sign + b_num.sign;

        let a_num = a_num >> a_num_offset_degree;
        let b_num = b_num >> b_num_offset_degree;

        Self::unsigned_multiply(a_num, b_num).set_sign(output_sign)
            << (a_num_offset_degree + b_num_offset_degree)
    }
}

impl Mul for RealNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::multiply(self, other)
    }
}

impl Mul<usize> for RealNumber {
    type Output = Self;

    fn mul(self, other: usize) -> Self::Output {
        self.mul(Self::from_usize(other))
    }
}

impl MulAssign for RealNumber {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone().mul(other);
    }
}

impl MulAssign<usize> for RealNumber {
    fn mul_assign(&mut self, other: usize) {
        *self = self.clone().mul(other);
    }
}
