use crate::sign::Sign;
use crate::RealNumber;
use std::cmp::Ordering;

impl RealNumber {
    pub(crate) fn ueq(&self, other: &Self) -> bool {
        self.clone()
            .set_sign(Sign::Positive)
            .eq(&other.clone().set_sign(Sign::Positive))
    }
}

impl PartialEq for RealNumber {
    fn eq(&self, other: &Self) -> bool {
        if let Ordering::Equal = self.cmp(other) {
            true
        } else {
            false
        }
    }
}

impl PartialEq<usize> for RealNumber {
    fn eq(&self, other: &usize) -> bool {
        self.eq(&Self::from_usize(*other))
    }
}

impl Eq for RealNumber {}
