use std::cmp::{Eq, Ordering, PartialEq};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Sign {
    Neg,
    Pos,
}

impl Sign {
    pub(crate) fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Neg, Self::Neg) | (Self::Pos, Self::Pos) => Ordering::Equal,
            (Self::Pos, Self::Neg) => Ordering::Greater,
            (Self::Neg, Self::Pos) => Ordering::Less,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloatNumber {
    exponent: isize,
    significand: Vec<u32>,
    sign: Sign,
}

type Parts<'c> = (Option<&'c [char]>, Option<&'c [char]>);

impl FloatNumber {
    pub(crate) const RADIX: u32 = 10;

    pub(crate) fn from_bounds(start_bound: isize, end_bound: isize) -> Self {
        Self {
            exponent: start_bound,
            significand: vec![0; (end_bound - start_bound).unsigned_abs()],
            sign: Sign::Pos,
        }
    }

    fn split_string_to_parts<'c>(string: String) -> Parts<'c> {
        let chars = string.chars().collect::<Vec<_>>();
        let mut parts = chars.split(|&sym| sym == '.');

        let int_part = parts.next();
        let frac_part = parts.next();

        if parts.next().is_some() {
            return (None, None);
        }

        (int_part, frac_part)
    }

    fn is_valid_part(part: &[char]) -> bool {
        part.len() != 0
            && part
                .iter()
                .fold(true, |acc, &sym| acc && sym.is_digit(Self::RADIX))
    }

    fn translate_part_to_significand(part: &[char]) -> Vec<u32> {
        part.iter()
            .map(|&sym| sym.to_digit(Self::RADIX).unwrap())
            .collect()
    }

    fn build_frac_num<'c>(parts: Parts<'c>) -> Option<Self> {
        let (int_part, frac_part) = match parts {
            (Some(int_part), Some(frac_part))
                if Self::is_valid_part(int_part) && Self::is_valid_part(frac_part) =>
            {
                (int_part, frac_part)
            }
            _ => return None,
        };

        let exponent = frac_part.len() as isize;
        let significand = [
            Self::translate_part_to_significand(int_part),
            Self::translate_part_to_significand(frac_part),
        ]
        .concat();

        Some(Self {
            significand,
            exponent,
            sign: Sign::Pos,
        })
    }

    fn build_int_num<'c>(parts: Parts<'c>) -> Option<Self> {
        let int_part = match parts {
            (Some(int_part), None) if Self::is_valid_part(int_part) => int_part,
            _ => return None,
        };

        let exponent = 0;
        let significand = Self::translate_part_to_significand(int_part);

        Some(Self {
            exponent,
            significand,
            sign: Sign::Pos,
        })
    }

    /// Creates new float number from unsigned numeric string.
    ///
    /// # Example
    ///
    /// ```
    /// let valid_string = String::from("123.456");
    /// let invalid_string = String::from("abc");
    ///
    /// let valid_num = FloatNumber::from_string(valid_string);
    /// let invalid_num = FloatNumber::from_string(invalid_string);
    ///
    /// assert_eq!(true, valid_num.is_some());
    /// assert_eq!(true, invalid_num.is_none());
    /// ```
    ///
    pub fn from_string(unsigned_numeric_string: String) -> Option<Self> {
        let parts = Self::split_string_to_parts(unsigned_numeric_string);

        let num = if let Some(num) = Self::build_frac_num(parts) {
            num
        } else {
            Self::build_int_num(parts)?
        };

        Some(num.trim_zeros())
    }

    /// Creates new zero float number.
    ///
    /// # Example
    ///
    /// ```
    /// let num = FloatNumber::zero();
    ///
    /// assert_eq!(true, num.is_zero());
    /// ```
    ///
    pub(crate) fn zero() -> Self {
        Self {
            significand: vec![0],
            exponent: 0,
            sign: Sign::Pos,
        }
    }

    /// Checks if the float number is zero.
    ///
    /// # Example
    ///
    /// ```
    /// let num = FloatNumber::from_string("0.000".to_string())?;
    ///
    /// assert_eq!(true, num.is_zero());
    /// ```
    ///
    pub(crate) fn is_zero(&self) -> bool {
        match self.significand.first() {
            Some(0) if self.significand.len() == 1 => true,
            _ => false,
        }
    }

    fn trim_left_zeros(mut self) -> Self {
        self.significand.reverse();

        while match self.significand.pop() {
            Some(0) if self.significand.len() != 0 => true,
            Some(sym) => {
                self.significand.push(sym);
                false
            }
            None => false,
        } {}

        self.significand.reverse();

        self
    }

    fn zeros_to_exp(mut self) -> Self {
        while match self.significand.pop() {
            Some(0) if self.significand.len() != 0 => {
                self.exponent += 1;
                true
            }
            Some(digit) => {
                self.significand.push(digit);
                false
            }
            None => {
                self.significand.push(0);
                false
            }
        } {}

        self
    }

    /// Trims the float number zeros.
    ///
    /// # Example
    ///
    /// ## Source
    ///
    /// **exponent: 0**
    ///
    /// | pos    | 15    | 14    | 13    | 12    | 11    | 10    | 9     | 8     | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 0     | 0     | 6     | 0     | 3     | 3     | 5     | 2     | 7     | 4     | 9     | 0     | 0     | 0     | 0     | 0     |
    ///
    /// ## Destination
    ///
    /// **exponent: 5**
    ///
    /// | pos    | 8    | 7    | 6    | 5    | 4     | 3     | 2     | 1     | 0     |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 6     | 0     | 3     | 3     | 5     | 2     | 7     | 4     | 9     |
    ///
    pub(crate) fn trim_zeros(mut self) -> Self {
        self.trim_left_zeros().zeros_to_exp()
    }

    /// Updates the float number ['Sign'].
    ///
    /// [`Sign`]: Sign
    ///
    /// # Example
    ///
    /// ```
    /// let mut num = FloatNumber::from_string("234234".to_string())?;
    ///
    /// assert_eq!(num.is_neg(), false);
    /// num.set_sign(Sign::Neg);
    /// assert_eq!(num.is_neg(), true);
    /// ```
    ///
    pub(crate) fn set_sign(mut self, sign: Sign) -> Self {
        self.sign = sign;
        self
    }

    /// Updates the float number ['Sign'] with the ['Sign'] of other float number.
    ///
    /// [`Sign`]: Sign
    ///
    /// # Example
    ///
    /// ```
    /// let a_num = FloatNumber::from_string("234234".to_string())?.set_sign(Sign::neg);
    /// let b_num = FloatNumber::from_string("234234".to_string())?.set_sign_of(a_num);
    ///
    /// assert_eq!(num.is_neg(), true);
    /// ```
    ///
    pub(crate) fn set_sign_of(mut self, other: Self) -> Self {
        self.sign = other.sign;
        self
    }

    /// Returns [`Ordering`] between self [`Sign`] and other [`Sign`].
    ///
    /// [`Sign`]: float_number::Sign
    ///
    /// # Example
    ///
    /// ```
    /// let a_num = FloatNumber::from_string("234234".to_string()).set_sign(Sign::Neg)?;
    /// let b_num = FloatNumber::from_string("341".to_string()).set_sign(Sign::Pos)?;
    /// let c_num = FloatNumber::from_string("13129".to_string()).set_sign(Sign::Pos)?;
    ///
    /// assert_eq!(Ordering::Less, a_num.cmp_sign(&b_num));
    /// assert_eq!(Ordering::Greater, b_num.cmp_sign(&a_num));
    /// assert_eq!(Ordering::Equal, b_num.cmp_sign(&c_num));
    /// ```
    ///
    pub(crate) fn cmp_sign(&self, other: &Self) -> Ordering {
        self.sign.cmp(&other.sign)
    }

    /// Checks if the float number is negative.
    ///
    /// # Example
    ///
    /// ```
    /// let num = FloatNumber::from_string("234234".to_string()).set_sign(Sign::Neg)?;
    ///
    /// assert_eq!(true num.is_neg());
    /// ```
    ///
    pub(crate) fn is_neg(&self) -> bool {
        match self.sign {
            Sign::Neg => true,
            _ => false,
        }
    }

    fn set_significand_item_value(&mut self, new_item_value: u32, pos: usize) {
        for _ in self.significand.len()..=pos {
            self.significand.push(0)
        }

        match self.significand.get_mut(pos) {
            Some(current_item) => *current_item = new_item_value,
            _ => (),
        }
    }

    fn significand_len(&self) -> isize {
        self.significand.len() as isize
    }

    /// Converts position.
    ///
    /// *significand element pos* **(=>)** *digit pos*
    /// **or**
    /// *digit pos* **(=>)** *significand element pos*
    ///
    /// # Explanations
    ///
    /// **exponent: -4**
    ///
    /// | pos         | 0     | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     | 9     | 10    | 11    |
    /// |-------------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | significand | 3     | 3     | 1     | 6     | 5     | 4     | 8     | 7     | 5     | 0     | 2     | 1     |
    ///
    /// | pos    | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |       | -1    | -2    | -3    | -4    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | 3     | 1     | 6     | 5     | 4     | 8     | 7     | .     | 5     | 0     | 2     | 1     |
    ///
    /// **exponent: 5**
    ///
    /// | pos         | 0     | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     |
    /// |-------------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | significand | 6     | 0     | 3     | 3     | 5     | 2     | 7     | 4     | 9     |
    ///
    /// | pos    | 13    | 12    | 11    | 10    | 9     | 8     | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 6     | 0     | 3     | 3     | 5     | 2     | 7     | 4     | 9     | 0     | 0     | 0     | 0     | 0     |
    ///
    /// **exponent: -8**
    ///
    /// | pos         | 0     | 1     | 2     | 3     |
    /// |-------------| :---: | :---: | :---: | :---: |
    /// | significand | 3     | 3     | 2     | 9     |
    ///
    /// | pos    | 0     |       | -1    | -2    | -3    | -4    | -5    | -6    | -7    | -8    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | .     | 0     | 0     | 0     | 0     | 3     | 3     | 2     | 9     |
    ///
    fn convert_pos(&self, pos: isize) -> isize {
        self.significand_len() + self.exponent - pos - 1
    }

    /// Updates digit value.
    ///
    /// # Panics
    ///
    /// Panics if digit radix is not equal to default radix of float number.
    ///
    /// # Example
    /// ```
    /// let mut num = FloatNumber::from_string("1234.5678".to_string())?;
    ///
    /// assert_eq!(Some(5), num.get_digit(-1));
    ///
    /// num.set_digit(8, -1);
    ///
    /// assert_eq!(Some(8), num.get_digit(-1));
    /// ```
    ///
    pub(crate) fn set_digit(&mut self, digit: u32, pos: isize) {
        if digit >= Self::RADIX {
            panic!("Digit radix is not equal to default radix of float number");
        }

        let pos = self.convert_pos(pos);

        let pos = if pos >= 0 {
            self.set_significand_item_value(digit, pos.unsigned_abs());
            return;
        } else {
            pos.unsigned_abs()
        };

        self.significand.reverse();
        self.set_significand_item_value(digit, self.significand.len() + pos - 1);
        self.significand.reverse();
    }

    /// Returns digit value by position.
    ///
    /// # Example
    /// ```
    /// let num = FloatNumber::from_string("1234.5678".to_string())?;
    ///
    /// assert_eq!(num.get_digit(0), Some(4));
    /// assert_eq!(num.get_digit(-15), None);
    /// ```
    ///
    pub(crate) fn get_digit(&self, pos: isize) -> Option<u32> {
        let pos = self.convert_pos(pos);

        if pos >= 0 {
            self.significand.get(pos.unsigned_abs()).map(|d| *d)
        } else {
            None
        }
    }

    /// Returns start bound.
    ///
    /// # Examples
    ///
    /// **start_bound = -4**
    ///
    /// | pos    | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |       | -1    | -2    | -3    | -4    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | 3     | 1     | 6     | 5     | 4     | 8     | 7     | .     | 5     | 0     | 2     | 1     |
    ///
    /// **start_bound = 0**
    ///
    /// | pos    | 13    | 12    | 11    | 10    | 9     | 8     | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 6     | 0     | 3     | 3     | 5     | 2     | 7     | 4     | 9     | 0     | 0     | 0     | 0     | 0     |
    ///
    /// **start_bound = -8**
    ///
    /// | pos    | 0     |       | -1    | -2    | -3    | -4    | -5    | -6    | -7    | -8    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | .     | 0     | 0     | 0     | 0     | 3     | 3     | 2     | 9     |
    ///
    pub(crate) fn start_bound(&self) -> isize {
        self.exponent
    }

    /// Returns end bound.
    ///
    /// # Examples
    ///
    /// **end_bound = 8**
    ///
    /// | pos    | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |       | -1    | -2    | -3    | -4    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | 3     | 1     | 6     | 5     | 4     | 8     | 7     | .     | 5     | 0     | 2     | 1     |
    ///
    /// **end_bound = 14**
    ///
    /// | pos    | 13    | 12    | 11    | 10    | 9     | 8     | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 6     | 0     | 3     | 3     | 5     | 2     | 7     | 4     | 9     | 0     | 0     | 0     | 0     | 0     |
    ///
    /// **end_bound = 1**
    ///
    /// | pos    | 0     |       | -1    | -2    | -3    | -4    | -5    | -6    | -7    | -8    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | .     | 0     | 0     | 0     | 0     | 3     | 3     | 2     | 9     |
    ///
    pub(crate) fn end_bound(&self) -> isize {
        self.convert_pos(0) + 1
    }

    /// Returns length of integer part.
    ///
    /// # Examples
    ///
    /// **int_part_len = 8**
    ///
    /// | pos    | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |       | -1    | -2    | -3    | -4    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | 3     | 1     | 6     | 5     | 4     | 8     | 7     | .     | 5     | 0     | 2     | 1     |
    ///
    /// **int_part_len = 14**
    ///
    /// | pos    | 13    | 12    | 11    | 10    | 9     | 8     | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 6     | 0     | 3     | 3     | 5     | 2     | 7     | 4     | 9     | 0     | 0     | 0     | 0     | 0     |
    ///
    /// **int_part_len = 1**
    ///
    /// | pos    | 0     |       | -1    | -2    | -3    | -4    | -5    | -6    | -7    | -8    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | .     | 0     | 0     | 0     | 0     | 3     | 3     | 2     | 9     |
    ///
    pub(crate) fn int_part_len(&self) -> usize {
        self.end_bound().unsigned_abs()
    }
}
