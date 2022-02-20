use crate::RealNumber;
use std::ops::{Add, AddAssign};

impl Add<RealNumber> for RealNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::sum(self, other)
    }
}

impl Add<usize> for RealNumber {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        self.add(Self::from_usize(other))
    }
}

impl AddAssign for RealNumber {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone().add(other);
    }
}

impl AddAssign<usize> for RealNumber {
    fn add_assign(&mut self, other: usize) {
        *self = self.clone().add(other);
    }
}
