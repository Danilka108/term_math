use std::cmp::{Eq, PartialEq, Ordering};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Sign {
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

impl FloatNumber {
    pub(crate) const RADIX: u32 = 10;

    fn new(significand: Vec<u32>, exponent: isize, sign: Sign) -> Self {
        Self {
            significand,
            exponent,
            sign,
        }
        .trim_zeros()
    }

    pub fn from_unsigned_numeric_string(unsigned_numeric_string: String) -> Option<Self> {
        let chars = unsigned_numeric_string.chars().collect::<Vec<_>>();
        let mut parts = chars.split(|&sym| sym == '.');

        let int_part = parts.next();
        let frac_part = parts.next();

        if parts.next().is_some() {
            return None;
        }

        let (exponent, significand) = match (int_part, frac_part) {
            (Some(int_part), Some(frac_part)) if int_part.len() != 0 && frac_part.len() != 0 => {
                let exponent = -(frac_part.len() as isize);
                let significand = [int_part, frac_part].concat();
                (exponent, significand)
            }
            (Some(int_part), None) if int_part.len() != 0 => (0, int_part.to_vec()),
            _ => return None,
        };

        let are_digits_valid = significand
            .iter()
            .fold(true, |acc, &sym| acc && sym.is_digit(Self::RADIX));

        if !are_digits_valid {
            return None;
        }

        let significand = significand
            .iter()
            .map(|&sym| sym.to_digit(Self::RADIX).unwrap())
            .collect();

        Some(Self::new(significand, exponent, Sign::Pos))
    }

    pub(crate) fn zero() -> Self {
        Self {
            significand: vec![0],
            exponent: 0,
            sign: Sign::Pos,
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

    pub(crate) fn trim_zeros(mut self) -> Self {
        self.trim_left_zeros().zeros_to_exp()
    }

    pub(crate) fn set_sign(mut self, sign: Sign) -> Self {
        self.sign = sign;
        self
    }

    pub(crate) fn cmp_sign(&self, other: &Self) -> Ordering {
        self.sign.cmp(&other.sign)
    }

    pub(crate) fn is_neg(&self) -> bool {
        match self.sign {
            Sign::Neg => true,
            _ => false
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

    fn convert_pos(&self, pos: isize) -> isize {
        self.significand_len() + self.exponent - pos - 1
    }

    pub(crate) fn set_digit(&mut self, digit: u32, pos: isize) {
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

    pub(crate) fn get_digit(&self, pos: isize) -> Option<u32> {
        let pos = self.convert_pos(pos);

        if pos >= 0 {
            self.significand.get(pos.unsigned_abs()).map(|d| *d)
        } else {
            None
        }
    }

    pub(crate) fn start_bound(&self) -> isize {
        self.exponent
    }

    pub(crate) fn end_bound(&self) -> isize {
        self.convert_pos(0) + 1
    }

    pub(crate) fn int_part_len(&self) -> usize {
        self.end_bound().unsigned_abs()
    }
}
