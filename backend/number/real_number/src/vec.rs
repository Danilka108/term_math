use super::constants::{DECIMAL_RADIX, DECIMAL_RADIX_I16, ZERO_AS_U8};

pub(crate) trait PushDigit {
    fn push_digit(&mut self, digit: u8);
    fn push_digit_i16(&mut self, digit: i16);
}

pub(crate) trait GetDigit {
    fn get_digit(&self, index: usize) -> Option<u8>;
    fn get_digit_i16(&self, index: usize) -> Option<i16>;
}

pub(crate) trait Shortest {
    fn shortest<'a>(&'a self, other: &'a Self) -> &'a Self;
}

pub(crate) trait Longest {
    fn longest<'a>(&'a self, other: &'a Self) -> &'a Self;
}

impl PushDigit for Vec<char> {
    fn push_digit(&mut self, digit: u8) {
        self.push((digit % DECIMAL_RADIX + ZERO_AS_U8) as char);
    }

    fn push_digit_i16(&mut self, digit: i16) {
        self.push(((digit % DECIMAL_RADIX_I16) as u8 + ZERO_AS_U8) as char);
    }
}

impl GetDigit for Vec<char> {
    fn get_digit(&self, index: usize) -> Option<u8> {
        let digit = *self.get(index)? as u8 - ZERO_AS_U8;
        Some(digit % DECIMAL_RADIX)
    }

    fn get_digit_i16(&self, index: usize) -> Option<i16> {
        let digit = *self.get(index)? as u8 - ZERO_AS_U8;

        Some((digit % DECIMAL_RADIX) as i16)
    }
}

impl Shortest for Vec<char> {
    fn shortest<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.len() < other.len() {
            self
        } else {
            other
        }
    }
}

impl Longest for Vec<char> {
    fn longest<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.len() >= other.len() {
            self
        } else {
            other
        }
    }
}
