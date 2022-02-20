use crate::RealNumber;
use std::ops::{Sub, SubAssign};

impl Sub for RealNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl Sub<usize> for RealNumber {
    type Output = Self;

    fn sub(self, other: usize) -> Self::Output {
        self.sub(Self::from_usize(other))
    }
}

impl SubAssign for RealNumber {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone().sub(other);
    }
}

impl SubAssign<usize> for RealNumber {
    fn sub_assign(&mut self, other: usize) {
        *self = self.clone().sub(other);
    }
}
