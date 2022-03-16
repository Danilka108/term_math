use std::cmp::{Eq, Ordering, PartialEq};
use std::ops::{Bound, RangeBounds};

trait ToSignificand {
    fn to_significand<const RADIX: u32>(self) -> Vec<u32>;
}

trait GetSignedLen {
    fn signed_len(&self) -> isize;
}

pub(crate) trait SetDigit<P> {
    fn set_digit(&mut self, new_digit_value: u32, pos: P);
}

pub(crate) trait GetDigit<P> {
    fn get_digit(&self, pos: P) -> Option<u32>;
}

pub(crate) trait SetSign<S> {
    fn set_sign(self, s: S) -> Self;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Sign {
    Neg,
    Pos,
}

#[derive(Clone, Debug)]
pub struct FloatNumber<const RADIX: u32> {
    exponent: isize,
    significand: Vec<u32>,
    sign: Sign,
}

impl ToSignificand for Vec<char> {
    fn to_significand<const RADIX: u32>(self) -> Vec<u32> {
        self.iter()
            .map(|&sym| sym.to_digit(RADIX).unwrap())
            .collect()
    }
}

impl SetDigit<usize> for Vec<u32> {
    fn set_digit(&mut self, new_digit_value: u32, pos: usize) {
        for _ in self.len()..=pos {
            self.push(0)
        }

        match self.get_mut(pos) {
            Some(current_item) => *current_item = new_digit_value,
            _ => (),
        }
    }
}

impl GetSignedLen for Vec<u32> {
    fn signed_len<'c>(&self) -> isize {
        self.len() as isize
    }
}

impl Sign {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Neg, Self::Neg) | (Self::Pos, Self::Pos) => Ordering::Equal,
            (Self::Pos, Self::Neg) => Ordering::Greater,
            (Self::Neg, Self::Pos) => Ordering::Less,
        }
    }
}

impl<const RADIX: u32> FloatNumber<RADIX> {
    pub(crate) fn from_bounds(start_bound: isize, end_bound: isize) -> Self {
        Self {
            significand: vec![0; (end_bound - start_bound).unsigned_abs()],
            sign: Sign::Pos,
            exponent: start_bound,
        }
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
    pub(crate) fn from_string(src: String) -> Option<Self> {
        let (int_part, frac_part): (Vec<_>, _) = src.chars().partition(|sym| sym == &'.');

        for part in [&int_part, &frac_part] {
            let is_part_valid = part
                .iter()
                .fold(true, |acc, &sym| acc && sym.is_digit(RADIX));

            if !is_part_valid {
                return None;
            };
        }

        if int_part.len() != 0 && frac_part.len() != 0 {
            Some(Self {
                exponent: -(frac_part.len() as isize),
                significand: [
                    int_part.to_significand::<RADIX>(),
                    frac_part.to_significand::<RADIX>(),
                ]
                .concat(),
                sign: Sign::Pos,
            })
        } else if int_part.len() != 0 {
            Some(Self {
                exponent: 0,
                significand: int_part.to_significand::<RADIX>(),
                sign: Sign::Pos,
            })
        } else {
            None
        }
    }

    pub(crate) fn zero() -> Self {
        Self {
            significand: vec![0],
            sign: Sign::Pos,
            exponent: 0,
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        match self.significand.first() {
            Some(0) if self.significand.len() == 1 => true,
            _ => false,
        }
    }

    fn trim_left_zeros(mut self) -> Self {
        self.significand.reverse();

        while match self.significand.pop() {
            Some(0) if self.significand.len() != 0 => {
                true
            }
            Some(sym) => {
                self.significand.push(sym);
                false
            }
            None => false,
        } {}

        self.significand.reverse();

        self
    }

    fn trim_right_zeros(mut self) -> Self {
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

    pub(crate) fn trim_zeros(self) -> Self {
        self.trim_left_zeros().trim_right_zeros()
    }

    pub(crate) fn cmp_sign(&self, other: &Self) -> Ordering {
        self.sign.cmp(&other.sign)
    }

    pub(crate) fn is_neg(&self) -> bool {
        match self.sign {
            Sign::Neg => true,
            _ => false,
        }
    }

    pub(crate) fn int_part_len(&self) -> isize {
        self.convert_pos(0) + 1
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
        self.significand.signed_len() + self.exponent  - pos - 1
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
}

impl<const RADIX: u32> SetSign<Sign> for FloatNumber<RADIX> {
    fn set_sign(mut self, sign: Sign) -> Self {
        self.sign = sign;
        self
    }
}

impl<const RADIX: u32> SetSign<Self> for FloatNumber<RADIX> {
    fn set_sign(mut self, other: Self) -> Self {
        self.sign = other.sign;
        self
    }
}

impl<const RADIX: u32> SetDigit<isize> for FloatNumber<RADIX> {
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
    fn set_digit(&mut self, digit: u32, pos: isize) {
        if digit >= RADIX {
            panic!("radix of digit is not equal to radix of float number");
        }

        let pos = self.convert_pos(pos);

        let pos = if pos >= 0 {
            self.significand.set_digit(digit, pos.unsigned_abs());
            return;
        } else {
            pos.unsigned_abs()
        };

        self.significand.reverse();
        self.significand
            .set_digit(digit, self.significand.len() + pos - 1);
        self.significand.reverse();
    }
}

impl<const RADIX: u32> GetDigit<isize> for FloatNumber<RADIX> {
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
    fn get_digit(&self, pos: isize) -> Option<u32> {
        let pos = self.convert_pos(pos);

        if pos >= 0 {
            self.significand.get(pos.unsigned_abs()).map(|d| *d)
        } else {
            None
        }
    }
}
