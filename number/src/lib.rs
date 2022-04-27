mod finite;
mod finite_cmp;
mod finite_ops;
mod number;
mod sign;

pub use finite::TryFromStrError;
pub use number::Number;

pub type Dec64 = number::Number<10, 64>;

impl Dec64 {
    pub const INFINITY: number::Number<10, 64> = Self {
        kind: number::NumberKind::Inf,
    };

    pub const NEG_INFINITY: number::Number<10, 64> = Self {
        kind: number::NumberKind::NegInf,
    };

    pub const NAN: number::Number<10, 64> = Self {
        kind: number::NumberKind::NaN,
    };
}
