mod constants;

mod digit_stack;
mod parse;
mod sign;
mod trim;
mod vec;

mod cmp;
mod ops;

use crate::sign::Sign;
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct RealNumber {
    integer_part: Vec<char>,
    fractional_part: Vec<char>,
    sign: Sign,
}

impl ToString for RealNumber {
    fn to_string(&self) -> String {
        [
            if self.sign == Sign::Negative {
                String::from("-")
            } else {
                String::from("")
            },
            self.integer_part.iter().collect::<String>(),
            String::from("."),
            self.fractional_part.iter().collect::<String>(),
        ]
        .concat()
    }
}

impl RealNumber {
    fn new(integer_part: Vec<char>, fractional_part: Vec<char>, sign: Sign) -> Self {
        Self {
            integer_part,
            fractional_part,
            sign,
        }
    }

    fn from_usize(val: usize) -> Self {
        Self {
            integer_part: val.to_string().chars().collect(),
            fractional_part: vec!['0'],
            sign: Sign::Positive,
        }
    }

    fn len(&self) -> usize {
        self.integer_part.len() + self.fractional_part.len()
    }

    fn int_len(&self) -> usize {
        self.integer_part.len()
    }

    fn frac_len(&self) -> usize {
        self.fractional_part.len()
    }

    fn zero() -> Self {
        Self {
            integer_part: vec!['0'],
            fractional_part: vec!['0'],
            sign: Sign::Positive,
        }
    }

    fn set_sign(mut self, sign: Sign) -> Self {
        self.sign = sign;
        self
    }
}
