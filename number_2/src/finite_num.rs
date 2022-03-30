use crate::sign::Sign;
use std::cmp::Ordering;
use std::ops::Range;

pub trait SetDigit<D, P> {
    fn set_digit(&mut self, new_digit_value: D, pos: P);
}

pub trait GetDigit<D, P> {
    fn get_digit(&self, pos: P) -> Option<D>;
}

pub trait SetSign<S> {
    fn set_sign(self, s: S) -> Self;
}

pub trait ReverseSign {
    fn reverse_sign(self) -> Self;
}

pub trait CmpSign {
    fn cmp_sign(&self, rhs: &Self) -> Ordering;

    fn is_neg(&self) -> bool;

    fn is_pos(&self) -> bool;
}

pub trait Bounds {
    fn merge_bounds(&self, other: &Self) -> Range<isize>;

    fn bounds(&self) -> Range<isize>;

    fn start_bound(&self) -> isize;

    fn end_bound(&self) -> isize;
}

pub trait One {
    fn one() -> Self;

    fn is_one(&self) -> bool;
}

pub trait Zero {
    fn zero() -> Self;

    fn neg_zero() -> Self;

    fn is_zero(&self) -> bool;
}

pub trait TrimZeros {
    fn trim_right_zeros(self) -> Self;

    fn trim_left_zeros(self) -> Self;

    fn trim_zeros(self) -> Self;
}

pub trait Len {
    fn int_len(&self) -> isize;

    fn frac_len(&self) -> isize;

    fn len(&self) -> isize;
}

pub trait ShiftPoint<EXP> {
    fn shift_point(self, offset: EXP) -> Self;
}

pub trait FiniteNum<'t, DIGIT, EXP, const RADIX: u32, const PRECISION: usize>
where
    Self: TryFrom<&'t str>
        + TryFrom<String>
        + From<DIGIT>
        + SetDigit<DIGIT, EXP>
        + GetDigit<DIGIT, EXP>
        + SetSign<Sign>
        + SetSign<Self>
        + ReverseSign
        + CmpSign
        + Bounds
        + One
        + Zero
        + TrimZeros
        + Len
        + ShiftPoint<EXP>,
{
}
