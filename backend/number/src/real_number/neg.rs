use super::RealNumber;
use std::ops::Neg;

impl Neg for RealNumber {
    type Output = RealNumber;

    fn neg(mut self) -> Self::Output {
        self.sign = -(self.sign);
        self
    }
}
