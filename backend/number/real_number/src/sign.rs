use std::ops::Add;
use std::ops::Neg;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

impl Add for Sign {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Positive, Self::Positive) | (Self::Negative, Self::Negative) => Self::Positive,
            _ => Self::Negative,
        }
    }
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Negative => Self::Positive,
            Self::Positive => Self::Negative,
        }
    }
}

impl PartialEq for Sign {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Positive, Self::Positive) | (Self::Negative, Self::Negative) => true,
            _ => false,
        }
    }
}

impl Eq for Sign {}
