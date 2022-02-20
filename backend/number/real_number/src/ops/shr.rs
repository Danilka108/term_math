use crate::RealNumber;
use std::collections::VecDeque;
use std::ops::{Shr, ShrAssign};

impl Shr<usize> for RealNumber {
    type Output = Self;

    fn shr(self, mut offset_number: usize) -> Self {
        let mut int_part = VecDeque::from(self.integer_part.clone());
        let mut frac_part = VecDeque::from(self.fractional_part.clone());

        while offset_number > 0 {
            offset_number -= 1;

            let chr = if let Some(chr) = frac_part.pop_front() {
                chr
            } else {
                '0'
            };

            int_part.push_back(chr);
        }

        if frac_part.len() == 0 {
            frac_part.push_back('0');
        }

        Self::new(Vec::from(int_part), Vec::from(frac_part), self.sign).trim_int_zeros()
    }
}

impl ShrAssign<usize> for RealNumber {
    fn shr_assign(&mut self, offset_number: usize) {
        *self = self.clone().shr(offset_number);
    }
}
