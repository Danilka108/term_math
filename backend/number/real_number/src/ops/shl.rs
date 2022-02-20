use crate::RealNumber;
use std::collections::VecDeque;
use std::ops::{Shl, ShlAssign};

impl Shl<usize> for RealNumber {
    type Output = Self;

    fn shl(self, mut offset_number: usize) -> Self::Output {
        let mut int_part = VecDeque::from(self.integer_part.clone());
        let mut frac_part = VecDeque::from(self.fractional_part.clone());

        while offset_number > 0 {
            offset_number -= 1;

            let chr = if let Some(chr) = int_part.pop_back() {
                chr
            } else {
                '0'
            };

            frac_part.push_front(chr);
        }

        if int_part.len() == 0 {
            int_part.push_back('0');
        }

        Self::new(Vec::from(int_part), Vec::from(frac_part), self.sign).trim_frac_zeros()
    }
}

impl ShlAssign<usize> for RealNumber {
    fn shl_assign(&mut self, offset_number: usize) {
        *self = self.clone().shl(offset_number);
    }
}
