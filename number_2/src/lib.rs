mod finite_num;
mod float_number;
mod number;
mod sign;

pub use float_number::TryFromStringError;
pub use number::Number;

pub struct NumberContext<const RADIX: u32, const PRECISION: usize>();

impl<'f, const RADIX: u32, const PRECISION: usize> NumberContext<RADIX, PRECISION> {
    pub fn from_string(
        &self,
        src: String,
    ) -> Result<Number<'f, RADIX, PRECISION>, TryFromStringError> {
        match Number::try_from(src) {
            Err(e) => Err(e as TryFromStringError),
            Ok(n) => Ok(n as Number<RADIX, PRECISION>),
        }
    }

    pub fn from_str(
        &self,
        src: &str,
    ) -> Result<Number<'f, RADIX, PRECISION>, TryFromStringError> {
        match Number::try_from(src) {
            Err(e) => Err(e as TryFromStringError),
            Ok(n) => Ok(n as Number<RADIX, PRECISION>),
        }
    }

    pub fn pos_inf(&self) -> Number<'f, RADIX, PRECISION> {
        Number::pos_inf()
    }

    pub fn neg_inf(&self) -> Number<'f, RADIX, PRECISION> {
        Number::neg_inf()
    }

    pub fn nan(&self) -> Number<'f, RADIX, PRECISION> {
        Number::nan()
    }
}
