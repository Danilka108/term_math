use crate::finite_num::*;
use crate::sign::Sign;
use std::cmp::Ordering;
use std::ops::{Bound, Range};

const SIGNIFICAND_START_INDEX: isize = 0;

trait ToSignificand {
    fn to_significand<const RADIX: u32>(self) -> Result<Vec<u32>, TryFromStringError>;
}

impl SetDigit<u32, usize> for Vec<u32> {
    fn set_digit(&mut self, new_digit_value: u32, pos: usize) {
        for _ in self.len()..=pos {
            self.push(0)
        }

        match self.get_mut(pos) {
            Some(current_item) => {
                *current_item = new_digit_value;
            }
            _ => (),
        }
    }
}

impl ToSignificand for &str {
    fn to_significand<const RADIX: u32>(self) -> Result<Vec<u32>, TryFromStringError> {
        let is_valid = self
            .chars()
            .fold(true, |acc, sym| acc && sym.is_digit(RADIX));

        if !is_valid {
            return Err(TryFromStringError::InvalidDigit);
        }

        let significand = self
            .chars()
            .map(|sym| sym.to_digit(RADIX).unwrap())
            .collect();

        Ok(significand)
    }
}

#[derive(Debug, Clone)]
pub enum TryFromStringError {
    Empty,
    InvalidDigit,
    SeveralPoints,
    TooLong,
    PointWithoutFracPart,
}

#[derive(Clone, Debug)]
pub struct FlNum<const RADIX: u32, const PRECISION: usize> {
    exponent: isize,
    significand: Vec<u32>,
    sign: Sign,
}

impl<const RADIX: u32, const PRECISION: usize> FlNum<RADIX, PRECISION> {
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
        (self.significand.len() as isize) + self.exponent - pos - 1
    }

    fn from_raw_parts<'s>(
        raw_int_part: Option<&'s str>, raw_frac_part: Option<&'s str>,
    ) -> Result<Self, TryFromStringError> {
        let int_part = if let Some(raw_int_part) = raw_int_part {
            raw_int_part.to_significand::<RADIX>()?
        } else {
            return Err(TryFromStringError::Empty);
        };

        let frac_part = if let Some(raw_frac_part) = raw_frac_part {
            raw_frac_part.to_significand::<RADIX>()?
        } else {
            return Ok(Self {
                exponent: 0,
                significand: int_part,
                sign: Sign::Pos,
            });
        };

        let exponent = -(frac_part.len() as isize);
        let significand = [int_part, frac_part].concat();

        Ok(Self {
            exponent,
            significand,
            sign: Sign::Pos,
        })
    }
}

impl<const RADIX: u32, const PRECISION: usize> TryFrom<&str> for FlNum<RADIX, PRECISION> {
    type Error = TryFromStringError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        isize::try_from(src.len()).map_err(|_| TryFromStringError::TooLong)?;

        let mut chars = src.chars().peekable();
        let sign = match chars.peek() {
            Some(&sym) if sym == '+' => {
                chars.next();
                Sign::Pos
            }
            Some(&sym) if sym == '-' => {
                chars.next();
                Sign::Neg
            }
            _ => Sign::Pos,
        };

        let src = chars.collect::<String>();
        let mut parts = src.split(|sym| sym == '.');
        let (int_part, frac_part) = (parts.next(), parts.next());

        if parts.next().is_some() {
            return Err(TryFromStringError::SeveralPoints);
        }

        let num = Self::from_raw_parts(int_part, frac_part)?;

        if num.len().unsigned_abs() > PRECISION {
            return Err(TryFromStringError::TooLong);
        }

        Ok(num.trim_zeros().set_sign(sign))
    }
}

impl<const RADIX: u32, const PRECISION: usize> TryFrom<String> for FlNum<RADIX, PRECISION> {
    type Error = TryFromStringError;

    fn try_from(src: String) -> Result<Self, Self::Error> {
        Self::try_from(src.as_str())
    }
}

impl<const RADIX: u32, const PRECISION: usize> From<u32> for FlNum<RADIX, PRECISION> {
    fn from(mut src: u32) -> Self {
        let mut significand = Vec::new();

        while src != 0 {
            significand.push(src % RADIX);
            src /= RADIX;
        }

        significand.reverse();

        Self {
            significand,
            exponent: 0,
            sign: Sign::Pos,
        }
    }
}

impl<const RADIX: u32, const PRECISION: usize> ToString for FlNum<RADIX, PRECISION> {
    fn to_string(&self) -> String {
        let mut res = Vec::new();

        for pos in self.bounds() {
            let digit = self.get_digit(pos).unwrap_or(0);
            res.push(char::from_digit(digit, RADIX).unwrap());

            if pos == -1 {
                res.push('.');
            }
        }

        res.push(self.sign.to_char());
        res.reverse();

        res.iter().collect::<String>()
    }
}

impl<const RADIX: u32, const PRECISION: usize> One for FlNum<RADIX, PRECISION> {
    fn one() -> Self {
        Self {
            significand: vec![1],
            sign: Sign::Pos,
            exponent: 0,
        }
    }

    fn is_one(&self) -> bool {
        matches!(self.significand.first(), Some(1) if self.significand.len() == 1 && self.exponent == 0)
    }
}

impl<const RADIX: u32, const PRECISION: usize> Zero for FlNum<RADIX, PRECISION> {
    fn zero() -> Self {
        Self {
            significand: vec![0],
            sign: Sign::Pos,
            exponent: 0,
        }
    }

    fn neg_zero() -> Self {
        Self {
            significand: vec![0],
            sign: Sign::Neg,
            exponent: 0,
        }
    }

    fn is_zero(&self) -> bool {
        matches!(self.significand.first(), Some(0) if self.significand.len() == 1)
    }
}

impl<const RADIX: u32, const PRECISION: usize> TrimZeros for FlNum<RADIX, PRECISION> {
    fn trim_left_zeros(mut self) -> Self {
        self.significand.reverse();

        while let Some(digit) = self.significand.pop() {
            if digit == 0 && self.significand.len() != 0 {
                continue;
            }

            self.significand.push(digit);
            break;
        }

        self.significand.reverse();

        self
    }

    fn trim_right_zeros(mut self) -> Self {
        while let Some(digit) = self.significand.pop() {
            if digit == 0 && self.significand.len() != 0 {
                self.exponent += 1;

                continue;
            }

            self.significand.push(digit);
            break;
        }

        self
    }

    fn trim_zeros(self) -> Self {
        self.trim_left_zeros().trim_right_zeros()
    }
}

impl<const RADIX: u32, const PRECISION: usize> CmpSign for FlNum<RADIX, PRECISION> {
    fn cmp_sign(&self, other: &Self) -> Ordering {
        self.sign.cmp(&other.sign)
    }

    fn is_neg(&self) -> bool {
        matches!(&self.sign, Sign::Neg)
    }

    fn is_pos(&self) -> bool {
        matches!(&self.sign, Sign::Pos)
    }
}

impl<const RADIX: u32, const PRECISION: usize> ReverseSign for FlNum<RADIX, PRECISION> {
    fn reverse_sign(mut self) -> Self {
        self.sign = match self.sign {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos,
        };

        self
    }
}

impl<const RADIX: u32, const PRECISION: usize> Len for FlNum<RADIX, PRECISION> {
    fn int_len(&self) -> isize {
        self.end_bound()
    }

    fn frac_len(&self) -> isize {
        -self.start_bound()
    }

    fn len(&self) -> isize {
        self.end_bound() - self.start_bound()
    }
}

impl<const RADIX: u32, const PRECISION: usize> ShiftPoint<isize> for FlNum<RADIX, PRECISION> {
    /// Shifts poitnt.
    ///
    /// Shifts to the right if the offset is positive.
    /// Shifts to the left if the offset is negative.
    fn shift_point(mut self, offset: isize) -> Self {
        self.exponent += offset;
        self
    }
}

impl<const RADIX: u32, const PRECISION: usize> SetSign<Sign> for FlNum<RADIX, PRECISION> {
    fn set_sign(mut self, sign: Sign) -> Self {
        self.sign = sign;
        self
    }
}

impl<const RADIX: u32, const PRECISION: usize> SetSign<Self> for FlNum<RADIX, PRECISION> {
    fn set_sign(mut self, other: Self) -> Self {
        self.sign = other.sign;
        self
    }
}

impl<const RADIX: u32, const PRECISION: usize> SetDigit<u32, Bound<&isize>>
    for FlNum<RADIX, PRECISION>
{
    fn set_digit(&mut self, digit: u32, pos: Bound<&isize>) {
        let pos = match pos {
            Bound::Included(p) => *p,
            Bound::Excluded(p) => p + 1,
            Bound::Unbounded => 0,
        };

        self.set_digit(digit, pos)
    }
}

impl<const RADIX: u32, const PRECISION: usize> SetDigit<u32, isize> for FlNum<RADIX, PRECISION> {
    fn set_digit(&mut self, digit: u32, pos: isize) {
        debug_assert!(
            digit < RADIX,
            "radix of digit is not equal to radix of float number"
        );

        if pos < self.exponent {
            self.exponent = pos;
        }

        let pos = self.convert_pos(pos);

        if pos >= 0 {
            self.significand.set_digit(digit, pos.unsigned_abs());
            return;
        }

        let pos = pos.unsigned_abs();

        self.significand.reverse();
        self.significand
            .set_digit(digit, self.significand.len() - 1 + pos);
        self.significand.reverse();
    }
}

impl<const RADIX: u32, const PRECISION: usize> GetDigit<u32, isize> for FlNum<RADIX, PRECISION> {
    fn get_digit(&self, pos: isize) -> Option<u32> {
        let pos = self.convert_pos(pos);

        if pos >= 0 {
            self.significand.get(pos.unsigned_abs()).map(|d| *d)
        } else {
            None
        }
    }
}

impl<const RADIX: u32, const PRECISION: usize> Bounds for FlNum<RADIX, PRECISION> {
    fn merge_bounds(&self, other: &Self) -> Range<isize> {
        let merged_start_bound = self.start_bound().min(other.start_bound());
        let merged_end_bound = self.end_bound().max(other.end_bound());

        merged_start_bound..merged_end_bound
    }

    fn bounds(&self) -> Range<isize> {
        self.start_bound()..self.end_bound()
    }

    fn start_bound(&self) -> isize {
        if self.exponent < 0 {
            self.exponent
        } else {
            0
        }
    }

    fn end_bound(&self) -> isize {
        let end_bound = self.convert_pos(SIGNIFICAND_START_INDEX) + 1;

        if end_bound <= 0 {
            1
        } else {
            end_bound
        }
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> FiniteNum<'f, u32, isize, RADIX, PRECISION>
    for FlNum<RADIX, PRECISION>
{
}
