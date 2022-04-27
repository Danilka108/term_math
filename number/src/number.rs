use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::finite::{Finite, FiniteOverflow, TryFromStrError};
use crate::finite_cmp::finite_cmp;
use crate::finite_ops::{finite_add, finite_div, finite_mul, finite_neg};
use crate::sign::Sign;

#[derive(Clone, Debug)]
pub(crate) enum NumberKind<const RADIX: u32, const PREC: isize> {
    Finite(Finite<RADIX, PREC>),
    Inf,
    NegInf,
    NaN,
}

#[derive(Clone, Debug)]
pub struct Number<const RADIX: u32, const PREC: isize> {
    pub(crate) kind: NumberKind<RADIX, PREC>,
}

impl<const RADIX: u32, const PREC: isize> TryFrom<&str> for Number<RADIX, PREC> {
    type Error = TryFromStrError<RADIX, PREC>;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let finite = match Finite::try_from(src) {
            Err(err) => return Err(err as Self::Error),
            Ok(f) => f as Finite<RADIX, PREC>,
        };

        Ok(Self::finite(finite))
    }
}

impl<const RADIX: u32, const PREC: isize> TryFrom<String> for Number<RADIX, PREC> {
    type Error = TryFromStrError<RADIX, PREC>;

    fn try_from(src: String) -> Result<Self, Self::Error> {
        Self::try_from(src.as_str())
    }
}

impl<const RADIX: u32, const PREC: isize> ToString for Number<RADIX, PREC> {
    fn to_string(&self) -> String {
        match &self.kind {
            NumberKind::NaN => "NaN".to_string(),
            NumberKind::NegInf => "-inf".to_string(),
            NumberKind::Inf => "inf".to_string(),
            NumberKind::Finite(finite) => finite.to_string(),
        }
    }
}

impl<const RADIX: u32, const PREC: isize> Number<RADIX, PREC> {
    fn inf() -> Self {
        Self {
            kind: NumberKind::<RADIX, PREC>::Inf,
        }
    }

    fn neg_inf() -> Self {
        Self {
            kind: NumberKind::<RADIX, PREC>::NegInf,
        }
    }

    fn nan() -> Self {
        Self {
            kind: NumberKind::<RADIX, PREC>::NaN,
        }
    }

    pub fn is_nan(&self) -> bool {
        matches!(self.kind, NumberKind::NaN)
    }

    fn finite(finite: Finite<RADIX, PREC>) -> Self {
        Self {
            kind: NumberKind::<RADIX, PREC>::Finite(finite),
        }
    }

    fn sign(&self) -> Option<Sign> {
        let sign = match &self.kind {
            NumberKind::Inf => Sign::Pos,
            NumberKind::NegInf => Sign::Neg,
            NumberKind::Finite(f) if f.is_neg() => Sign::Neg,
            NumberKind::Finite(_) => Sign::Pos,
            NumberKind::NaN => return None,
        };

        Some(sign)
    }

    fn set_sign(self, sign: Sign) -> Self {
        match self.kind {
            NumberKind::Inf | NumberKind::NegInf => match sign {
                Sign::Pos => Self::inf(),
                Sign::Neg => Self::neg_inf(),
            },
            NumberKind::NaN => Self::nan(),
            NumberKind::Finite(f) => Self::finite(f.set_sign(&sign)),
        }
    }

    pub fn zero() -> Self {
        Self::finite(Finite::zero())
    }

    pub fn one() -> Self {
        Self::finite(Finite::one())
    }
}

trait OverflowToInf<const RADIX: u32, const PREC: isize> {
    fn overflow_to_inf(self) -> Number<RADIX, PREC>;
}

impl<const RADIX: u32, const PREC: isize> Default for Number<RADIX, PREC> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const RADIX: u32, const PREC: isize> OverflowToInf<RADIX, PREC>
    for Result<Finite<RADIX, PREC>, FiniteOverflow>
{
    fn overflow_to_inf(self) -> Number<RADIX, PREC> {
        match self {
            Ok(finite) => Number::finite(finite),
            Err(overflow) => match overflow.sign {
                Sign::Pos => Number::inf(),
                Sign::Neg => Number::neg_inf(),
            },
        }
    }
}

impl<const RADIX: u32, const PREC: isize> PartialEq for Number<RADIX, PREC> {
    fn eq(&self, rhs: &Self) -> bool {
        matches!(self.partial_cmp(rhs), Some(Ordering::Equal))
    }

    fn ne(&self, rhs: &Self) -> bool {
        matches!(
            self.partial_cmp(rhs),
            Some(Ordering::Less | Ordering::Greater)
        )
    }
}

impl<const RADIX: u32, const PREC: isize> PartialOrd for Number<RADIX, PREC> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let ord = match (&self.kind, &rhs.kind) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => return None,
            (NumberKind::Inf, NumberKind::Inf) | (NumberKind::NegInf, NumberKind::NegInf) => {
                Ordering::Equal
            }
            (NumberKind::NegInf, _) | (_, NumberKind::Inf) => Ordering::Less,
            (NumberKind::Inf, _) | (_, NumberKind::NegInf) => Ordering::Greater,
            (NumberKind::Finite(lhs), NumberKind::Finite(rhs)) => finite_cmp(lhs, rhs),
        };

        Some(ord)
    }
}

impl<const RADIX: u32, const PREC: isize> Neg for Number<RADIX, PREC> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.kind {
            NumberKind::NegInf => Self::inf(),
            NumberKind::Inf => Self::neg_inf(),
            NumberKind::NaN => Self::nan(),
            NumberKind::Finite(f) => Self::finite(finite_neg(f)),
        }
    }
}

impl<const RADIX: u32, const PREC: isize> Add for Number<RADIX, PREC> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.kind, rhs.kind) {
            (NumberKind::Inf, NumberKind::NegInf)
            | (NumberKind::NegInf, NumberKind::Inf)
            | (NumberKind::NaN, _)
            | (_, NumberKind::NaN) => Self::nan(),

            (NumberKind::Inf, _) | (_, NumberKind::Inf) => Self::inf(),
            (NumberKind::NegInf, _) | (_, NumberKind::NegInf) => Self::neg_inf(),

            (NumberKind::Finite(lhs_finite), NumberKind::Finite(rhs_finite)) => {
                finite_add(lhs_finite, rhs_finite).overflow_to_inf()
            }
        }
    }
}

impl<const RADIX: u32, const PREC: isize> AddAssign for Number<RADIX, PREC> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone().add(rhs);
    }
}

impl<const RADIX: u32, const PREC: isize> Sub for Number<RADIX, PREC> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl<const RADIX: u32, const PREC: isize> SubAssign for Number<RADIX, PREC> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone().sub(rhs);
    }
}

impl<const RADIX: u32, const PREC: isize> Mul for Number<RADIX, PREC> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (lhs_sign, rhs_sign) = (self.sign(), rhs.sign());

        let res_sign = match (lhs_sign.unwrap(), rhs_sign.unwrap()) {
            (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => Sign::Pos,
            (Sign::Pos, Sign::Neg) | (Sign::Neg, Sign::Pos) => Sign::Neg,
        };

        match (self.kind, rhs.kind) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => return Self::nan(),
            (NumberKind::Inf | NumberKind::NegInf, _)
            | (_, NumberKind::Inf | NumberKind::NegInf) => Self::inf().set_sign(res_sign),
            (NumberKind::Finite(lhs_finite), NumberKind::Finite(rhs_finite)) => {
                finite_mul(lhs_finite, rhs_finite).overflow_to_inf()
            }
        }
    }
}

impl<const RADIX: u32, const PREC: isize> MulAssign for Number<RADIX, PREC> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone().mul(rhs);
    }
}

impl<const RADIX: u32, const PREC: isize> Div for Number<RADIX, PREC> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (lhs_sign, rhs_sign) = (self.sign(), rhs.sign());

        let res_sign = match (lhs_sign.unwrap(), rhs_sign.unwrap()) {
            (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => Sign::Pos,
            (Sign::Pos, Sign::Neg) | (Sign::Neg, Sign::Pos) => Sign::Neg,
        };

        match (self.kind, rhs.kind) {
            (NumberKind::NaN, _)
            | (_, NumberKind::NaN)
            | (NumberKind::Inf | NumberKind::NegInf, NumberKind::Inf | NumberKind::NegInf) => {
                Self::nan()
            }
            (NumberKind::Finite(_), NumberKind::Inf | NumberKind::NegInf) => {
                Self::zero().set_sign(res_sign)
            }
            (NumberKind::Inf | NumberKind::NegInf, NumberKind::Finite(_)) => {
                Self::inf().set_sign(res_sign)
            }
            (NumberKind::Finite(lhs_finite), NumberKind::Finite(rhs_finite)) => {
                finite_div(lhs_finite, rhs_finite).overflow_to_inf()
            }
        }
    }
}

impl<const RADIX: u32, const PREC: isize> DivAssign for Number<RADIX, PREC> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone().div(rhs);
    }
}
