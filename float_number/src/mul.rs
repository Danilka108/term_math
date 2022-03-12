use crate::FloatNumber;
use crate::Sign;

impl FloatNumber {
    fn umul_to_digit(&self, other_digit: u32) -> Self {
        let self_start_bound = self.start_bound();
        let self_end_bound = self.end_bound();

        let mut new_num = FloatNumber {
            significand: vec![0; (self_end_bound - self_start_bound).unsigned_abs()],
            exponent: self_start_bound,
            sign: Sign::Pos,
        };

        let mut buffer = 0;
        for self_pos in self_start_bound..self_end_bound {
            let self_digit = self.get_digit(self_pos).unwrap();
            let mul = self_digit * other_digit + buffer;
            new_num.set_digit(mul % Self::RADIX, self_pos);
            buffer = mul / Self::RADIX;
        }

        if buffer != 0 {
            new_num.set_digit(buffer, self_end_bound);
        }

        new_num.trim_left_zeros().zeros_to_exp()
    }

    fn umul(mut self, mut other: Self) -> Self {
        let new_num_exp = self.exponent + other.exponent;

        self.exponent = 0;
        other.exponent = 0;

        let other_start_bound = other.start_bound();
        let other_end_bound = other.end_bound();

        let mut new_num = Self::zero();

        for other_pos in other_start_bound..other_end_bound {
            let other_digit = other.get_digit(other_pos).unwrap();
            let mut mul = self.umul_to_digit(other_digit);
            mul.exponent = other_pos;
            new_num = new_num.sum(mul);
        }

        new_num.exponent = new_num_exp;
        new_num.trim_left_zeros().zeros_to_exp()
    }

    pub fn mul(self, other: Self) -> Self {
        let result_sign = match (&self.sign, &other.sign) {
            (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => Sign::Pos,
            (Sign::Pos, Sign::Neg) | (Sign::Neg, Sign::Pos) => Sign::Neg,
        };

        self.umul(other).set_sign(result_sign)
    }
}
