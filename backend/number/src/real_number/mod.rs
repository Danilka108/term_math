mod constants;
mod digit_stack;
mod add;
mod neg;
mod sub;
mod eq;
mod ord;

use crate::sign::Sign;

#[derive(Clone, Debug)]
pub struct RealNumber {
    integer_part: Vec<char>,
    fractional_part: Vec<char>,
    sign: Sign,
}

impl RealNumber {
    pub(crate) fn new(integer_part: Vec<char>, fractional_part: Vec<char>, sign: Sign) -> Self {
        Self {
            integer_part,
            fractional_part,
            sign,
        }
    }

    fn set_sign(mut self, sign: Sign) -> Self {
        self.sign = sign;
        self
    }
}
