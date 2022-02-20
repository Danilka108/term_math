use crate::number::{Number, NumberKind};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign, Neg};

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self.kind, other.kind) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => Self::get_nan(),
            (NumberKind::Real(s), NumberKind::Real(o)) => Self::get_real(s + o),
        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self.kind, other.kind) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => Self::get_nan(),
            (NumberKind::Real(s), NumberKind::Real(o)) => Self::get_real(s - o),
        }
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.kind {
            NumberKind::NaN => Self::get_nan(),
            NumberKind::Real(s) => Self::get_real(-s),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self.kind, other.kind) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => Self::get_nan(),
            (NumberKind::Real(s), NumberKind::Real(o)) => Self::get_real(s * o),
        }
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let (s, o) = match (self.kind, other.kind) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => return Self::get_nan(),
            (NumberKind::Real(s), NumberKind::Real(o)) => (s, o),
        };

        if o == 0 {
            return Self::get_nan();
        }

        Self::get_real(s / o)
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}
