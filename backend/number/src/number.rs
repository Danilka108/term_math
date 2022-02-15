use crate::real_number::RealNumber;
use crate::sign::Sign;
use ast::NumberNode;
use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Clone, Debug)]
pub enum Number {
    NaN,
    Real(RealNumber),
}

impl Number {
    pub fn from_number_node(number_node: NumberNode) -> Self {
        let val = number_node.value().chars().collect::<Vec<_>>();
        let parts = val.split(|chr| *chr == '.').collect::<Vec<_>>();

        if parts.len() == 0 || parts.len() > 2 {
            return Self::NaN;
        }

        let mut int_part = Vec::new();
        let mut frac_part = Vec::new();

        for (i, part) in parts.iter().enumerate() {
            let is_valid = part.iter().fold(true, |acc, chr| acc && chr.is_digit(10));

            if !is_valid || (i == 0 && part.len() == 0) {
                return Self::NaN;
            }

            match i {
                0 if part[0] == '0' && part.len() > 1 => return Self::NaN,
                0 => int_part = part.to_vec(),
                1 => frac_part = part.to_vec(),
                _ => return Self::NaN,
            }
        }

        Self::Real(RealNumber::new(int_part, frac_part, Sign::Positive))
    }

    pub fn is_nan(&self) -> bool {
        if let Self::NaN = self {
            true
        } else {
            false
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        if self.is_nan() | other.is_nan() {
            false
        } else {
            self.eq(other)
        }
    }

    fn ne(&self, other: &Self) -> bool {
        if self.is_nan() | other.is_nan() {
            false
        } else {
            self.ne(other)
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nan() | other.is_nan() {
            None
        } else {
            self.partial_cmp(other)
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.is_nan() | other.is_nan() {
            Self::NaN
        } else {
            self + other
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.is_nan() | other.is_nan() {
            Self::NaN
        } else {
            self + other
        }
    }
}
