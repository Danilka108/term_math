use super::RealNumber;

impl RealNumber {
    pub(crate) fn trim_frac_zeros(mut self) -> Self {
        let mut frac_part = Vec::new();
        let mut is_insignificant_zeros = true;

        for &chr in self.fractional_part.iter().rev() {
            if chr == '0' && is_insignificant_zeros {
                continue;
            }

            is_insignificant_zeros = false;
            frac_part.push(chr)
        }

        if frac_part.len() == 0 {
            frac_part.push('0');
        }

        frac_part.reverse();
        self.fractional_part = frac_part;

        self
    }

    pub(crate) fn trim_int_zeros(mut self) -> Self {
        let mut int_part = Vec::new();
        let mut is_insignificant_zeros = true;

        for &chr in self.integer_part.iter() {
            if chr == '0' && is_insignificant_zeros {
                continue;
            }

            is_insignificant_zeros = false;
            int_part.push(chr)
        }

        if int_part.len() == 0 {
            int_part.push('0');
        }

        self.integer_part = int_part;

        self
    }
}
