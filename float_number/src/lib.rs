mod cmp;
mod from;
mod mul;
mod sum;

use std::cmp::{Eq, PartialEq};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Sign {
    Neg,
    Pos,
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
        .trim_left_zeros()
        .zeros_to_exp()
    }

    fn zero() -> Self {
        Self {
            significand: vec![0],
            exponent: 0,
            sign: Sign::Pos,
        }
    }

    fn is_zero(&self) -> bool {
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

    fn set_sign(mut self, sign: Sign) -> Self {
        self.sign = sign;
        self
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

    fn set_digit(&mut self, digit: u32, pos: isize) {
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

    fn get_digit(&self, pos: isize) -> Option<u32> {
        let pos = self.convert_pos(pos);

        if pos >= 0 {
            self.significand.get(pos.unsigned_abs()).map(|d| *d)
        } else {
            None
        }
    }

    fn start_bound(&self) -> isize {
        self.exponent
    }

    fn end_bound(&self) -> isize {
        self.convert_pos(0) + 1
    }

    fn int_part_len(&self) -> usize {
        self.end_bound().unsigned_abs()
    }
}
