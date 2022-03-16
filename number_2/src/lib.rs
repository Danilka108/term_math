mod float_number;

use float_number::FloatNumber;


#[derive(Clone, Debug)]
pub enum NumberKind<const RADIX: u32> {
    NaN,
    Float(FloatNumber<RADIX>),
}

#[derive(Clone, Debug)]
pub struct Number<const RADIX: u32>(NumberKind<RADIX>);
