use crate::finite_cmp::{finite_cmp, get_unsigned_finite_max_min, is_unsigned_finite_eq};
use crate::finite::{Finite, FiniteOverflow};
use crate::sign::Sign;
use std::cmp::Ordering;

fn check_to_overflow<const RADIX: u32, const PREC: isize>(
    num: &Finite<RADIX, PREC>,
) -> Result<(), FiniteOverflow> {
    match num.len().checked_add(1) {
        None if num.is_neg() => Err(FiniteOverflow { sign: Sign::Neg }),
        None => Err(FiniteOverflow { sign: Sign::Pos }),
        Some(len) if len > PREC && num.is_neg() => Err(FiniteOverflow { sign: Sign::Neg }),
        Some(len) if len > PREC => Err(FiniteOverflow { sign: Sign::Pos }),
        Some(_) => Ok(()),
    }
}

fn unsigned_add<const RADIX: u32, const PREC: isize>(
    lhs: Finite<RADIX, PREC>,
    rhs: Finite<RADIX, PREC>,
) -> Result<Finite<RADIX, PREC>, FiniteOverflow> {
    check_to_overflow(&lhs)?;

    let mut new_num = Finite::zero();
    let mut buffer = 0;

    for pos in lhs.merge_bounds(&rhs) {
        let lhs_digit = lhs.get_digit(pos).unwrap_or(0);
        let rhs_digit = rhs.get_digit(pos).unwrap_or(0);

        let sum = buffer + lhs_digit + rhs_digit;

        new_num.set_digit(sum % RADIX, pos);
        buffer = sum / RADIX;
    }

    if buffer != 0 {
        new_num.set_digit(buffer % RADIX, lhs.end_bound());
    }

    Ok(new_num.trim_zeros().set_sign_of(&lhs))
}

fn unsigned_sub<const RADIX: u32, const PREC: isize>(
    lhs: Finite<RADIX, PREC>,
    rhs: Finite<RADIX, PREC>,
) -> Result<Finite<RADIX, PREC>, FiniteOverflow> {
    let mut new_num = Finite::zero();
    let mut borrowing = 0;

    for pos in lhs.merge_bounds(&rhs) {
        let lhs_digit = lhs.get_digit(pos).unwrap_or(0);
        let rhs_digit = rhs.get_digit(pos).unwrap_or(0);

        let subtract = if lhs_digit < rhs_digit + borrowing {
            let sub = RADIX + lhs_digit - rhs_digit - borrowing;
            borrowing = 1;
            sub
        } else {
            let sub = lhs_digit - rhs_digit - borrowing;
            borrowing = 0;
            sub
        };

        new_num.set_digit(subtract, pos);
    }

    if borrowing != 0 {
        panic!("reduced number is less than subtracted number");
    }

    Ok(new_num.trim_zeros().set_sign_of(&lhs))
}

pub fn finite_add<const RADIX: u32, const PREC: isize>(
    lhs: Finite<RADIX, PREC>,
    rhs: Finite<RADIX, PREC>,
) -> Result<Finite<RADIX, PREC>, FiniteOverflow> {
    let (umax, umin) = get_unsigned_finite_max_min(lhs, rhs);

    match umax.cmp_sign(&umin) {
        Ordering::Equal => unsigned_add(umax, umin),
        _ if is_unsigned_finite_eq(&umax, &umin) => Ok(Finite::zero()),
        _ => unsigned_sub(umax, umin),
    }
}

pub fn finite_neg<const RADIX: u32, const PREC: isize>(num: Finite<RADIX, PREC>) -> Finite<RADIX, PREC> {
    num.reverse_sign()
}

fn unsigned_mul_to_digit<const RADIX: u32, const PREC: isize>(
    num: &Finite<RADIX, PREC>,
    digit: u32,
) -> Result<Finite<RADIX, PREC>, FiniteOverflow> {
    check_to_overflow(&num)?;

    let mut new_num = Finite::zero();
    let mut buffer = 0;

    for pos in num.bounds() {
        let self_digit = num.get_digit(pos).unwrap();
        let mul = self_digit * digit + buffer;

        new_num.set_digit(mul % RADIX, pos);
        buffer = mul / RADIX;
    }

    if buffer != 0 {
        new_num.set_digit(buffer, new_num.end_bound());
    }

    Ok(new_num.trim_zeros())
}

fn unsigned_mul<const RADIX: u32, const PREC: isize>(
    lhs: Finite<RADIX, PREC>,
    rhs: Finite<RADIX, PREC>,
) -> Result<Finite<RADIX, PREC>, FiniteOverflow> {
    let mut new_num = Finite::zero();

    for pos in rhs.bounds() {
        let rhs_digit = rhs.get_digit(pos).unwrap();
        let mul = unsigned_mul_to_digit(&lhs, rhs_digit)?;
        new_num = unsigned_add(new_num, mul)?;
    }

    Ok(new_num.trim_zeros())
}

pub fn finite_mul<const RADIX: u32, const PREC: isize>(
    lhs: Finite<RADIX, PREC>,
    rhs: Finite<RADIX, PREC>,
) -> Result<Finite<RADIX, PREC>, FiniteOverflow> {
    let res_sign = match lhs.cmp_sign(&rhs) {
        Ordering::Equal => Sign::Pos,
        _ => Sign::Neg,
    };

    if lhs.is_zero() || rhs.is_zero() {
        return Ok(Finite::zero().set_sign(&res_sign));
    }

    unsigned_mul(lhs, rhs)
        .map(|n| n.set_sign(&res_sign))
        .map_err(|e| e.set_sign(&res_sign))
}

fn unsigned_div<const RADIX: u32, const PREC: isize>(
    mut lhs: Finite<RADIX, PREC>,
    rhs: Finite<RADIX, PREC>,
) -> Result<Finite<RADIX, PREC>, FiniteOverflow> {
    let mut delta_len = lhs.len() - rhs.len();
    let mut quotient: Finite<RADIX, PREC> = Finite::zero();

    loop {
        let shifted_rhs = rhs.clone().shift_point(delta_len);

        if let Ordering::Less = finite_cmp(&lhs, &shifted_rhs) {
            delta_len -= 1;
            continue;
        }

        let subtract = unsigned_sub(lhs, shifted_rhs)?;

        let next_quotient = unsigned_add(quotient.clone(), Finite::one().shift_point(delta_len))?;

        if let Err(_) = check_to_overflow(&next_quotient) {
            break;
        }

        lhs = subtract;
        quotient = next_quotient;

        if lhs.is_zero() && delta_len < 0 {
            break;
        }
    }

    Ok(quotient.trim_zeros())
}

pub fn finite_div<const RADIX: u32, const PREC: isize>(
    lhs: Finite<RADIX, PREC>,
    rhs: Finite<RADIX, PREC>,
) -> Result<Finite<RADIX, PREC>, FiniteOverflow> {
    let res_sign = match lhs.cmp_sign(&rhs) {
        Ordering::Equal => Sign::Pos,
        _ => Sign::Neg,
    };

    if rhs.is_zero() {
        return Err(FiniteOverflow { sign: res_sign });
    }

    if lhs.is_zero() {
        return Ok(Finite::zero().set_sign(&res_sign));
    }

    if rhs.is_one() {
        return Ok(lhs.set_sign(&res_sign));
    }

    unsigned_div(lhs, rhs)
        .map(|n| n.set_sign(&res_sign))
        .map_err(|e| e.set_sign(&res_sign))
}
