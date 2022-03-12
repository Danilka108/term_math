use crate::FloatNumber;
use crate::Sign;

impl FloatNumber {
    fn uadd(self, other: Self) -> Self {
        let start_bound = self.start_bound().min(other.start_bound());
        let end_bound = self.end_bound().max(other.end_bound());

        let mut new_num = Self {
            significand: vec![0; (end_bound - start_bound).unsigned_abs()],
            exponent: start_bound,
            sign: Sign::Pos,
        };
        let mut buffer = 0;

        for pos in start_bound..end_bound {
            let self_digit = self.get_digit(pos).unwrap_or(0);
            let other_digit = other.get_digit(pos).unwrap_or(0);
            let sum = buffer + self_digit + other_digit;

            new_num.set_digit(sum % Self::RADIX, pos);
            buffer = sum / Self::RADIX;
        }

        if buffer != 0 {
            new_num.set_digit(buffer % Self::RADIX, end_bound);
        }

        new_num.trim_left_zeros().zeros_to_exp()
    }

    fn usub(self, other: Self) -> Self {
        let start_bound = self.start_bound().min(other.start_bound());
        let end_bound = self.end_bound().max(other.end_bound());

        let mut new_num = Self {
            significand: vec![0; (end_bound - start_bound).unsigned_abs()],
            exponent: start_bound,
            sign: Sign::Pos,
        };
        let mut borrowing = 0;

        for pos in start_bound..end_bound {
            let self_digit = self.get_digit(pos).unwrap_or(0);
            let other_digit = other.get_digit(pos).unwrap_or(0);

            let subtract = if self_digit < other_digit + borrowing {
                let sub = Self::RADIX + self_digit - other_digit - borrowing;
                borrowing = 1;
                sub
            } else {
                let sub = self_digit - other_digit - borrowing;
                borrowing = 0;
                sub
            };

            new_num.set_digit(subtract, pos);
        }

        if borrowing != 0 {
            panic!("Reduced number is less than subtracted number");
        }

        new_num.trim_left_zeros().zeros_to_exp()
    }

    pub fn sum(self, other: Self) -> Self {
        let (max, min) = self.get_umax_umin(other);
        let result_sign = max.sign.clone();

        if max.sign != min.sign && max.is_ueq(&min) {
            return Self::zero();
        }

        if max.sign == min.sign {
            max.uadd(min)
        } else {
            max.usub(min)
        }
        .set_sign(result_sign)
    }

    pub fn neg(self) -> Self {
        match self.sign {
            Sign::Pos => self.set_sign(Sign::Neg),
            Sign::Neg => self.set_sign(Sign::Pos),
        }
    }
}
