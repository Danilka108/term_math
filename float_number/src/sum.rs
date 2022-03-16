use crate::FloatNumber;

impl FloatNumber {
    fn uadd(max_num: Self, min_num: Self) -> Self {
        let start_bound = max_num.start_bound().min(min_num.start_bound());
        let end_bound = max_num.end_bound().max(min_num.end_bound());

        let mut new_num = FloatNumber::from_bounds(start_bound, end_bound);
        let mut buffer = 0;

        for pos in start_bound..end_bound {
            let max_num_digit = max_num.get_digit(pos).unwrap_or(0);
            let min_num_digit = min_num.get_digit(pos).unwrap_or(0);
            let sum = buffer + max_num_digit + min_num_digit;

            new_num.set_digit(sum % Self::RADIX, pos);
            buffer = sum / Self::RADIX;
        }

        if buffer != 0 {
            new_num.set_digit(buffer % Self::RADIX, end_bound);
        }

        new_num.trim_zeros().set_sign_of(max_num)
    }

    fn usub(reduced_num: Self, subtracted_num: Self) -> Self {
        let start_bound = reduced_num.start_bound().min(subtracted_num.start_bound());
        let end_bound = reduced_num.end_bound().max(subtracted_num.end_bound());

        let mut new_num = Self::from_bounds(start_bound, end_bound);
        let mut borrowing = 0;

        for pos in start_bound..end_bound {
            let reduced_digit = reduced_num.get_digit(pos).unwrap_or(0);
            let subtracted_digit = subtracted_num.get_digit(pos).unwrap_or(0);

            let subtract = if reduced_digit < subtracted_digit + borrowing {
                let sub = Self::RADIX + reduced_digit - subtracted_digit - borrowing;
                borrowing = 1;
                sub
            } else {
                let sub = reduced_digit - subtracted_digit - borrowing;
                borrowing = 0;
                sub
            };

            new_num.set_digit(subtract, pos);
        }

        if borrowing != 0 {
            panic!("Reduced number is less than subtracted number");
        }

        new_num.trim_zeros().set_sign_of(reduced_num)
    }

    pub fn sum(self, other: Self) -> Self {
        use std::cmp::Ordering;

        let (max, min) = self.get_umax_umin(other);

        match max.cmp_sign(&min) {
            Ordering::Equal => Self::uadd(max, min),
            _ if max.is_ueq(&min) => Self::zero(),
            _ => Self::usub(max, min),
        }
    }

    pub fn neg(self) -> Self {
        use crate::float_number::Sign;

        if self.is_neg() {
            self.set_sign(Sign::Pos)
        } else {
            self.set_sign(Sign::Neg)
        }
    }
}
