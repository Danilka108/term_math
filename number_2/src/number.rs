use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::finite_num::*;
use crate::float_number::{FlNum, TryFromStringError};
use crate::sign::Sign;

trait CheckToOverFlow<T> {
    type CheckRes;

    fn is_will_overflow(t: T) -> bool;

    fn check_to_overflow(t: T) -> Option<Self::CheckRes>;

    fn check_to_overflow_with_sign(t: T, sign: &Sign) -> Option<Self::CheckRes>;
}

#[derive(Clone, Debug)]
enum NumberKind<'f, F, const RADIX: u32, const PRECISION: usize>
where
    F: FiniteNum<'f, u32, isize, RADIX, PRECISION>,
{
    Finite(F, PhantomData<&'f F>),
    PosInf,
    NegInf,
    NaN,
}

#[derive(Clone, Debug)]
pub struct Number<'f, const RADIX: u32, const PRECISION: usize>(
    NumberKind<'f, FlNum<RADIX, PRECISION>, RADIX, PRECISION>,
);

impl<'f, const RADIX: u32, const PRECISION: usize> TryFrom<&str> for Number<'f, RADIX, PRECISION> {
    type Error = TryFromStringError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let finite = match FlNum::try_from(src) {
            Err(err) => return Err(err as Self::Error),
            Ok(f) => f as FlNum<RADIX, PRECISION>,
        };

        Ok(Self::finite(finite))
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> TryFrom<String>
    for Number<'f, RADIX, PRECISION>
{
    type Error = TryFromStringError;

    fn try_from(src: String) -> Result<Self, Self::Error> {
        Self::try_from(src.as_str())
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> ToString for Number<'f, RADIX, PRECISION> {
    fn to_string(&self) -> String {
        match &self.0 {
            NumberKind::NaN => "nan".to_string(),
            NumberKind::NegInf => "-inf".to_string(),
            NumberKind::PosInf => "inf".to_string(),
            NumberKind::Finite(finite, _) => finite.to_string(),
        }
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> CheckToOverFlow<&FlNum<RADIX, PRECISION>>
    for Number<'f, RADIX, PRECISION>
{
    type CheckRes = Self;

    fn is_will_overflow(num: &FlNum<RADIX, PRECISION>) -> bool {
        match num.len().checked_add(1) {
            Some(l) => l.unsigned_abs() > PRECISION,
            None => true,
        }
    }

    fn check_to_overflow(num: &FlNum<RADIX, PRECISION>) -> Option<Self::CheckRes> {
        if Self::is_will_overflow(num) && num.is_pos() {
            return Some(Self::pos_inf());
        }

        if Self::is_will_overflow(num) && num.is_neg() {
            return Some(Self::neg_inf());
        }

        None
    }

    fn check_to_overflow_with_sign(
        num: &FlNum<RADIX, PRECISION>,
        sign: &Sign,
    ) -> Option<Self::CheckRes> {
        if Self::is_will_overflow(num) && sign.is_pos() {
            return Some(Self::pos_inf());
        }

        if Self::is_will_overflow(num) && sign.is_neg() {
            return Some(Self::neg_inf());
        }

        None
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> CheckToOverFlow<&Self>
    for Number<'f, RADIX, PRECISION>
{
    type CheckRes = Self;

    fn is_will_overflow(num: &Self) -> bool {
        let finite = match &num.0 {
            NumberKind::Finite(f, _) => f,
            _ => return false,
        };

        Self::is_will_overflow(finite)
    }

    fn check_to_overflow(num: &Self) -> Option<Self::CheckRes> {
        let finite = match &num.0 {
            NumberKind::Finite(f, _) => f,
            _ => return None,
        };

        Self::check_to_overflow(finite)
    }

    fn check_to_overflow_with_sign(num: &Self, sign: &Sign) -> Option<Self::CheckRes> {
        let finite = match &num.0 {
            NumberKind::Finite(f, _) => f,
            _ => return None,
        };

        Self::check_to_overflow_with_sign(finite, sign)
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> Number<'f, RADIX, PRECISION> {
    pub(crate) fn pos_inf() -> Self {
        Self(NumberKind::<'f, FlNum<RADIX, PRECISION>, RADIX, PRECISION>::PosInf)
    }

    pub(crate) fn neg_inf() -> Self {
        Self(NumberKind::<'f, FlNum<RADIX, PRECISION>, RADIX, PRECISION>::NegInf)
    }

    pub(crate) fn nan() -> Self {
        Self(NumberKind::<'f, FlNum<RADIX, PRECISION>, RADIX, PRECISION>::NaN)
    }

    fn finite(finite: FlNum<RADIX, PRECISION>) -> Self {
        Self(
            NumberKind::<'f, FlNum<RADIX, PRECISION>, RADIX, PRECISION>::Finite(
                finite,
                PhantomData,
            ),
        )
    }

    pub(crate) fn zero() -> Self {
        Self::finite(FlNum::zero())
    }

    pub(crate) fn one() -> Self {
        Self::finite(FlNum::one())
    }

    fn map_finite(
        self,
        predicate: impl FnOnce(FlNum<RADIX, PRECISION>) -> FlNum<RADIX, PRECISION>,
    ) -> Self {
        match self.0 {
            NumberKind::Finite(f, _) => Self::finite(predicate(f)),
            k => Self(k),
        }
    }

    fn unsigned_finite_cmp(
        lhs: &FlNum<RADIX, PRECISION>,
        rhs: &FlNum<RADIX, PRECISION>,
    ) -> Ordering {
        match lhs.int_len().cmp(&rhs.int_len()) {
            Ordering::Equal => (),
            ord => return ord,
        }

        for pos in lhs.merge_bounds(&rhs).rev() {
            let lsh_digit = lhs.get_digit(pos).unwrap_or(0);
            let rhs_digit = rhs.get_digit(pos).unwrap_or(0);

            match lsh_digit.cmp(&rhs_digit) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }

        Ordering::Equal
    }

    fn finite_cmp(lhs: &FlNum<RADIX, PRECISION>, rhs: &FlNum<RADIX, PRECISION>) -> Ordering {
        if lhs.is_zero() && rhs.is_zero() {
            return Ordering::Equal;
        }

        match lhs.cmp_sign(&rhs) {
            Ordering::Equal if lhs.is_neg() => Self::unsigned_finite_cmp(lhs, rhs).reverse(),
            Ordering::Equal => Self::unsigned_finite_cmp(lhs, rhs),
            ord => ord,
        }
    }

    fn is_finite_eq(lhs: &FlNum<RADIX, PRECISION>, rhs: &FlNum<RADIX, PRECISION>) -> bool {
        matches!(Self::finite_cmp(lhs, rhs), Ordering::Equal)
    }

    fn unsigned_finite_max_min(
        lhs: FlNum<RADIX, PRECISION>,
        rhs: FlNum<RADIX, PRECISION>,
    ) -> (FlNum<RADIX, PRECISION>, FlNum<RADIX, PRECISION>) {
        match Self::unsigned_finite_cmp(&lhs, &rhs) {
            Ordering::Less => (rhs, lhs),
            Ordering::Greater | Ordering::Equal => (lhs, rhs),
        }
    }

    fn unsigned_finite_add(
        max_num: FlNum<RADIX, PRECISION>,
        min_num: FlNum<RADIX, PRECISION>,
    ) -> Self {
        let mut new_num = FlNum::zero();
        let mut buffer = 0;

        if let Some(n) = Self::check_to_overflow(&max_num) {
            return n;
        }

        for pos in max_num.merge_bounds(&min_num) {
            let max_num_digit = max_num.get_digit(pos).unwrap_or(0);
            let min_num_digit = min_num.get_digit(pos).unwrap_or(0);

            let sum = buffer + max_num_digit + min_num_digit;

            new_num.set_digit(sum % RADIX, pos);
            buffer = sum / RADIX;
        }

        if buffer != 0 {
            new_num.set_digit(buffer % RADIX, max_num.end_bound());
        }

        Self::finite(new_num.trim_zeros().set_sign(max_num))
    }

    fn unsigned_finite_sub(
        reduced_num: FlNum<RADIX, PRECISION>,
        subtracted_num: FlNum<RADIX, PRECISION>,
    ) -> Self {
        let mut new_num = FlNum::zero();
        let mut borrowing = 0;

        for pos in reduced_num.merge_bounds(&subtracted_num) {
            let reduced_digit = reduced_num.get_digit(pos).unwrap_or(0);
            let subtracted_digit = subtracted_num.get_digit(pos).unwrap_or(0);

            let subtract = if reduced_digit < subtracted_digit + borrowing {
                let sub = RADIX + reduced_digit - subtracted_digit - borrowing;
                borrowing = 1;
                sub
            } else {
                let sub = reduced_digit - subtracted_digit - borrowing;
                borrowing = 0;
                sub
            };

            new_num.set_digit(subtract, pos);
        }

        if borrowing != 0 {
            panic!("reduced number is less than subtracted number");
        }

        Self::finite(new_num.trim_zeros().set_sign(reduced_num))
    }

    fn finite_sum(lhs: FlNum<RADIX, PRECISION>, rhs: FlNum<RADIX, PRECISION>) -> Self {
        let (umax, umin) = Self::unsigned_finite_max_min(lhs, rhs);

        match umax.cmp_sign(&umin) {
            Ordering::Equal => Self::unsigned_finite_add(umax, umin),
            _ if Self::is_finite_eq(&umax, &umin) => Self::finite(FlNum::zero()),
            _ => Self::unsigned_finite_sub(umax, umin),
        }
    }

    fn unsigned_finite_mul_to_digit(
        res_sign: &Sign,
        num: &FlNum<RADIX, PRECISION>,
        digit: u32,
    ) -> Self {
        if let Some(n) = Self::check_to_overflow_with_sign(num, res_sign) {
            return n;
        }

        let mut new_num = FlNum::zero();
        let mut buffer = 0;

        for pos in num.bounds() {
            let num_digit = num.get_digit(pos).unwrap();
            let mul = num_digit * digit + buffer;

            new_num.set_digit(mul % RADIX, pos);
            dbg!(new_num.clone());
            buffer = mul / RADIX;
        }

        if buffer != 0 {
            new_num.set_digit(buffer, new_num.end_bound());
        }

        Self::finite(new_num.set_sign(Sign::Pos).trim_zeros())
    }

    fn finite_mul(lhs: FlNum<RADIX, PRECISION>, rhs: FlNum<RADIX, PRECISION>) -> Self {
        if lhs.is_zero() || rhs.is_zero() {
            return Self::finite(FlNum::zero());
        }

        let res_sign = match lhs.cmp_sign(&rhs) {
            Ordering::Equal => Sign::Pos,
            _ => Sign::Neg,
        };

        let mut new_num = Self::finite(FlNum::zero());

        for pos in rhs.bounds() {
            let digit = rhs.get_digit(pos).unwrap();

            let mul = Self::unsigned_finite_mul_to_digit(&res_sign, &lhs, digit)
                .map_finite(|f| f.shift_point(pos));

            new_num += mul;
        }

        new_num.map_finite(|f| f.trim_zeros().set_sign(res_sign))
    }

    fn finite_div(
        finite_dividend: FlNum<RADIX, PRECISION>,
        finite_divisor: FlNum<RADIX, PRECISION>,
    ) -> Self {
        if finite_divisor.is_one() {
            return Self::finite(finite_dividend);
        }

        let res_sign = match finite_dividend.cmp_sign(&finite_divisor) {
            Ordering::Equal => Sign::Pos,
            _ => Sign::Neg,
        };

        let mut dividend = Self::finite(finite_dividend.clone().set_sign(Sign::Pos));
        let divisor = Self::finite(finite_divisor.clone().set_sign(Sign::Pos));

        let mut delta_len = finite_dividend.len() - finite_divisor.len();
        let mut quotient = Self::zero();

        loop {
            let subtract =
                dividend.clone() - divisor.clone().map_finite(|f| f.shift_point(delta_len));

            if subtract < Self::zero() {
                delta_len -= 1;
                continue;
            }

            let next_quotient =
                quotient.clone() + Self::one().map_finite(|f| f.shift_point(delta_len));

            if let Some(_) = Self::check_to_overflow_with_sign(&next_quotient, &res_sign) {
                break;
            }

            dividend = subtract;
            quotient = next_quotient;

            if dividend == Self::zero() && delta_len < 0 {
                break;
            }
        }

        quotient.map_finite(|f| f.set_sign(res_sign))
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> PartialEq for Number<'f, RADIX, PRECISION> {
    fn eq(&self, rhs: &Self) -> bool {
        matches!(self.partial_cmp(rhs), Some(Ordering::Equal))
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> PartialOrd for Number<'f, RADIX, PRECISION> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let ord = match (&self.0, &rhs.0) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => return None,
            (NumberKind::PosInf, NumberKind::PosInf) | (NumberKind::NegInf, NumberKind::NegInf) => {
                Ordering::Equal
            }
            (NumberKind::NegInf, _) | (_, NumberKind::PosInf) => Ordering::Less,
            (NumberKind::PosInf, _) | (_, NumberKind::NegInf) => Ordering::Greater,
            (NumberKind::Finite(lhs, _), NumberKind::Finite(rhs, _)) => Self::finite_cmp(lhs, rhs),
        };

        Some(ord)
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> Neg for Number<'f, RADIX, PRECISION> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.0 {
            NumberKind::NegInf => Self::pos_inf(),
            NumberKind::PosInf => Self::neg_inf(),
            NumberKind::NaN => Self::nan(),
            NumberKind::Finite(f, _) => Self::finite(f.reverse_sign()),
        }
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> Add for Number<'f, RADIX, PRECISION> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (NumberKind::PosInf, NumberKind::NegInf)
            | (NumberKind::NegInf, NumberKind::PosInf)
            | (NumberKind::NaN, _)
            | (_, NumberKind::NaN) => Self::nan(),
            (NumberKind::PosInf, _) | (_, NumberKind::PosInf) => Self::pos_inf(),
            (NumberKind::NegInf, _) | (_, NumberKind::NegInf) => Self::neg_inf(),
            (NumberKind::Finite(lhs, _), NumberKind::Finite(rhs, _)) => Self::finite_sum(lhs, rhs),
        }
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> AddAssign for Number<'f, RADIX, PRECISION> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone().add(rhs);
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> Sub for Number<'f, RADIX, PRECISION> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> SubAssign for Number<'f, RADIX, PRECISION> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone().add(rhs.neg());
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> Mul for Number<'f, RADIX, PRECISION> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (NumberKind::NaN, _) | (_, NumberKind::NaN) => Self::nan(),
            (NumberKind::PosInf, NumberKind::PosInf) | (NumberKind::NegInf, NumberKind::NegInf) => {
                Self::pos_inf()
            }
            (NumberKind::PosInf, NumberKind::NegInf) | (NumberKind::NegInf, NumberKind::PosInf) => {
                Self::neg_inf()
            }
            (NumberKind::PosInf, _) | (_, NumberKind::PosInf) => Self::pos_inf(),
            (NumberKind::NegInf, _) | (_, NumberKind::NegInf) => Self::neg_inf(),
            (NumberKind::Finite(lhs, _), NumberKind::Finite(rhs, _)) => Self::finite_mul(lhs, rhs),
        }
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> MulAssign for Number<'f, RADIX, PRECISION> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone().mul(rhs);
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> Div for Number<'f, RADIX, PRECISION> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (NumberKind::NaN, _)
            | (_, NumberKind::NaN)
            | (NumberKind::PosInf, NumberKind::PosInf)
            | (NumberKind::NegInf, NumberKind::NegInf)
            | (NumberKind::PosInf, NumberKind::NegInf)
            | (NumberKind::NegInf, NumberKind::PosInf) => Self::nan(),

            // +inf / -finite = -inf; 
            (NumberKind::PosInf, NumberKind::Finite(rhs, _)) if rhs.is_neg() => Self::neg_inf(),
            // +inf / +finite = +inf; 
            (NumberKind::PosInf, NumberKind::Finite(___, _)) => Self::pos_inf(),
            // -finite / +inf = -0; 
            (NumberKind::Finite(lhs, _), NumberKind::PosInf) if lhs.is_neg() => Self::finite(FlNum::neg_zero()),
            // +finite / +inf = 0; 
            (NumberKind::Finite(___, _), NumberKind::PosInf) => Self::finite(FlNum::zero()),
            // -inf / -finite = +inf; 
            (NumberKind::NegInf, NumberKind::Finite(rhs, _)) if rhs.is_neg() => Self::pos_inf(),
            // -inf / +finite = -inf; 
            (NumberKind::NegInf, NumberKind::Finite(___, _)) => Self::neg_inf(),
            // -finite / -inf = -0; 
            (NumberKind::Finite(lhs, _), NumberKind::NegInf) if lhs.is_neg() => Self::finite(FlNum::zero()),
            // +finite / -inf = -0; 
            (NumberKind::Finite(___, _), NumberKind::NegInf) => Self::finite(FlNum::neg_zero()),

            (NumberKind::Finite(lhs, _), NumberKind::Finite(rhs, _))
                if lhs.is_zero() && rhs.is_zero() || lhs.is_zero() =>
            {
                Self::nan()
            }
            (NumberKind::Finite(lhs, _), NumberKind::Finite(rhs, _)) => Self::finite_div(lhs, rhs),
        }
    }
}

impl<'f, const RADIX: u32, const PRECISION: usize> DivAssign for Number<'f, RADIX, PRECISION> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone().div(rhs);
    }
}
