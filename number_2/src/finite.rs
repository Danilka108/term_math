use crate::sign::Sign;
use std::cmp::Ordering;
use std::ops::Range;

const MAX_PREC: isize = 100;

pub struct FiniteOverflow {
    pub sign: Sign,
}

#[derive(Debug, Clone)]
pub enum TryFromStrError {
    Empty,
    TooLong,
    InvalidRadix,
    InvalidDigit,
    InvalidPrecision,
    SeveralPoints,
    PointWithoutFracPart,
}

#[derive(Clone, Debug)]
pub struct Finite<const RADIX: u32, const PREC: isize> {
    mantissa: Vec<u32>,
    exp: isize,
    sign: Sign,
}

impl FiniteOverflow {
    pub fn set_sign(mut self, sign: &Sign) -> Self {
        self.sign = sign.clone();
        self
    }
}

impl<const RADIX: u32, const PREC: isize> Finite<RADIX, PREC> {
    fn raw_part_to_mantissa(raw_part: &str) -> Result<Vec<u32>, TryFromStrError> {
        let is_valid = raw_part
            .chars()
            .fold(true, |acc, sym| acc && sym.is_digit(RADIX));

        if !is_valid {
            return Err(TryFromStrError::InvalidDigit);
        }

        let mantissa = raw_part
            .chars()
            .map(|sym| sym.to_digit(RADIX).unwrap())
            .collect();

        Ok(mantissa)
    }

    fn from_raw_parts<'s>(
        raw_int_part: Option<&'s str>,
        raw_frac_part: Option<&'s str>,
    ) -> Result<Self, TryFromStrError> {
        let int_part = if let Some(raw_int_part) = raw_int_part {
            Self::raw_part_to_mantissa(raw_int_part)?
        } else {
            return Err(TryFromStrError::Empty);
        };

        let frac_part = if let Some(raw_frac_part) = raw_frac_part {
            Self::raw_part_to_mantissa(raw_frac_part)?
        } else {
            return Ok(Self {
                exp: 0,
                mantissa: int_part,
                sign: Sign::Pos,
            });
        };

        if frac_part.len() == 0 {
            return Err(TryFromStrError::PointWithoutFracPart);
        }

        let exp = -(frac_part.len() as isize);
        let mantissa = [int_part, frac_part].concat();

        Ok(Self {
            exp,
            mantissa,
            sign: Sign::Pos,
        })
    }
}

impl<const RADIX: u32, const PREC: isize> TryFrom<&str> for Finite<RADIX, PREC> {
    type Error = TryFromStrError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        if RADIX < 2 || RADIX > 36 {
            return Err(TryFromStrError::InvalidRadix);
        }

        if PREC <= 0 || PREC > MAX_PREC {
            return Err(TryFromStrError::InvalidPrecision);
        }

        if isize::try_from(src.len()).map_err(|_| TryFromStrError::TooLong)? > MAX_PREC {
            return Err(TryFromStrError::TooLong);
        }

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
            return Err(TryFromStrError::SeveralPoints);
        }

        let num = Self::from_raw_parts(int_part, frac_part)?;

        Ok(num.trim_zeros().set_sign(&sign))
    }
}

impl<const RADIX: u32, const PREC: isize> ToString for Finite<RADIX, PREC> {
    fn to_string(&self) -> String {
        let mut val = Vec::new();

        for pos in self.bounds() {
            let digit_as_char = char::from_digit(self.get_digit(pos).unwrap(), RADIX).unwrap();
            val.push(digit_as_char); 

            if pos == -1 {
                val.push('.');
            }
        }

        match self.sign {
            Sign::Neg => val.push('-'),
            _ => (),
        }

        val.reverse();

        val.iter().collect()
    }
}

impl<const RADIX: u32, const PREC: isize> Finite<RADIX, PREC> {
    fn trim_left_zeros(mut self) -> Self {
        self.mantissa.reverse();

        while let Some(digit) = self.mantissa.pop() {
            if digit == 0 && self.mantissa.len() != 0 {
                continue;
            }

            self.mantissa.push(digit);
            break;
        }

        self.mantissa.reverse();

        self
    }

    fn trim_right_zeros(mut self) -> Self {
        while let Some(digit) = self.mantissa.pop() {
            if digit == 0 && self.mantissa.len() != 0 {
                self.exp += 1;

                continue;
            }

            self.mantissa.push(digit);
            break;
        }

        self
    }

    pub fn trim_zeros(self) -> Self {
        self.trim_left_zeros().trim_right_zeros()
    }

    pub fn set_sign(mut self, sign: &Sign) -> Self {
        self.sign = sign.clone();
        self
    }

    pub fn set_sign_of(mut self, other: &Self) -> Self {
        self.sign = other.sign.clone();
        self
    }

    pub fn reverse_sign(mut self) -> Self {
        self.sign = self.sign.reverse();
        self
    }

    pub fn cmp_sign(&self, other: &Self) -> Ordering {
        match (&self.sign, &other.sign) {
            (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => Ordering::Equal,
            (Sign::Pos, Sign::Neg) => Ordering::Greater,
            (Sign::Neg, Sign::Pos) => Ordering::Less,
        }
    }

    pub fn is_pos(&self) -> bool {
        matches!(self.sign, Sign::Pos)
    }

    pub fn is_neg(&self) -> bool {
        matches!(self.sign, Sign::Neg)
    }

    pub fn zero() -> Self {
        Self {
            exp: 0,
            mantissa: vec![0],
            sign: Sign::Pos,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.mantissa.len() == 1 && matches!(self.mantissa.first(), Some(&0))
    }

    pub fn one() -> Self {
        Self {
            exp: 0,
            mantissa: vec![1],
            sign: Sign::Pos,
        }
    }

    pub fn is_one(&self) -> bool {
        self.mantissa.len() == 1 && matches!(self.mantissa.first(), Some(&1))
    }

    /// Converts position.
    ///
    /// *mantissa element pos* **(=>)** *digit pos*
    /// **or**
    /// *digit pos* **(=>)** *mantissa element pos*
    ///
    /// # Explanations
    ///
    /// **exponent: -4**
    ///
    /// | pos         | 0     | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     | 9     | 10    | 11    |
    /// |-------------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | mantissa | 3     | 3     | 1     | 6     | 5     | 4     | 8     | 7     | 5     | 0     | 2     | 1     |
    ///
    /// | pos    | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |       | -1    | -2    | -3    | -4    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | 3     | 1     | 6     | 5     | 4     | 8     | 7     | .     | 5     | 0     | 2     | 1     |
    ///
    /// **exponent: 5**
    ///
    /// | pos         | 0     | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     |
    /// |-------------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | mantissa | 6     | 0     | 3     | 3     | 5     | 2     | 7     | 4     | 9     |
    ///
    /// | pos    | 13    | 12    | 11    | 10    | 9     | 8     | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0     |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 6     | 0     | 3     | 3     | 5     | 2     | 7     | 4     | 9     | 0     | 0     | 0     | 0     | 0     |
    ///
    /// **exponent: -8**
    ///
    /// | pos         | 0     | 1     | 2     | 3     |
    /// |-------------| :---: | :---: | :---: | :---: |
    /// | mantissa | 3     | 3     | 2     | 9     |
    ///
    /// | pos    | 0     |       | -1    | -2    | -3    | -4    | -5    | -6    | -7    | -8    |
    /// |--------| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
    /// | number | 3     | .     | 0     | 0     | 0     | 0     | 3     | 3     | 2     | 9     |
    ///
    fn convert_pos(&self, pos: isize) -> isize {
        (self.mantissa.len() as isize) + self.exp - pos - 1
    }

    fn set_mantissa_element(&mut self, val: u32, pos: usize) {
        for _ in self.mantissa.len()..=pos {
            self.mantissa.push(0)
        }

        match self.mantissa.get_mut(pos) {
            Some(current_item) => {
                *current_item = val;
            }
            _ => (),
        }
    }

    pub fn set_digit(&mut self, digit: u32, pos: isize) {
        debug_assert!(
            digit < RADIX,
            "radix of digit is not equal to default radix"
        );

        if pos < self.exp {
            self.exp = pos;
        }

        let pos = self.convert_pos(pos);

        if pos >= 0 {
            self.set_mantissa_element(digit, pos.unsigned_abs());
            return;
        }

        let pos = pos.unsigned_abs();

        self.mantissa.reverse();
        self.set_mantissa_element(digit, self.mantissa.len() - 1 + pos);
        self.mantissa.reverse();
    }

    pub fn get_digit(&self, pos: isize) -> Option<u32> {
        let pos = self.convert_pos(pos);

        if pos >= 0 {
            self.mantissa.get(pos.unsigned_abs()).map(|d| *d)
        } else {
            None
        }
    }

    pub fn merge_bounds(&self, other: &Self) -> Range<isize> {
        let merged_start_bound = self.start_bound().min(other.start_bound());
        let merged_end_bound = self.end_bound().max(other.end_bound());

        merged_start_bound..merged_end_bound
    }

    pub fn bounds(&self) -> Range<isize> {
        self.start_bound()..self.end_bound()
    }

    pub fn start_bound(&self) -> isize {
        if self.exp < 0 {
            self.exp
        } else {
            0
        }
    }

    pub fn end_bound(&self) -> isize {
        let end_bound = self.convert_pos(0) + 1;

        if end_bound <= 0 {
            1
        } else {
            end_bound
        }
    }

    pub fn int_len(&self) -> isize {
        self.end_bound()
    }

    pub fn frac_len(&self) -> isize {
        -self.start_bound()
    }

    pub fn len(&self) -> isize {
        self.end_bound() - self.start_bound()
    }

    /// Shifts poitnt.
    ///
    /// Shifts to the right if the offset is positive.
    /// Shifts to the left if the offset is negative.
    pub fn shift_point(mut self, offset: isize) -> Self {
        self.exp += offset;
        self
    }
}
