use crate::constants::DEFAULT_FRAC_ACCURACY;
use constants::DECIMAL_RADIX;
use crate::RealNumber;
use std::ops::{Div, DivAssign};

impl RealNumber {
    fn get_offset_degrees(dividend: &Self, divisor: &Self) -> (usize, usize) {
        let frac_accuracy = dividend
            .fractional_part
            .len()
            .max(divisor.fractional_part.len())
            .max(DEFAULT_FRAC_ACCURACY);

        let dividend_offset_degree = dividend.fractional_part.len()
            + frac_accuracy
            + if dividend.len() < divisor.len() {
                divisor.len() - dividend.len()
            } else {
                0
            };

        let divisor_offset_degree = divisor.fractional_part.len();

        (dividend_offset_degree, divisor_offset_degree)
    }

    fn divide(mut dividend: Self, mut divisor: Self) -> Self {
        let output_sign = dividend.sign + divisor.sign;

        let (dividend_offset_degree, divisor_offset_degree) =
            Self::get_offset_degrees(&dividend, &divisor);

        dividend = dividend >> dividend_offset_degree;
        divisor = divisor >> divisor_offset_degree;

        let mut delta_len = dividend.integer_part.len() - divisor.integer_part.len();
        let mut quotient = Self::zero();

        loop {
            let subtract = dividend.clone() - (divisor.clone() >> delta_len);

            if subtract < 0 && delta_len == 0 {
                break;
            }

            if subtract < 0 {
                delta_len -= 1;
                continue;
            }

            dividend = subtract;
            quotient += usize::pow(DECIMAL_RADIX as usize, delta_len as u32);
        }

        (quotient >> divisor_offset_degree << dividend_offset_degree).set_sign(output_sign)
    }
}

impl Div for RealNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::divide(self, other)
    }
}

impl DivAssign for RealNumber {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone().div(other);
    }
}
