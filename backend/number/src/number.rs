use real_number::RealNumber;
use ast::node::NumNode;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub(crate) enum NumberKind {
    NaN,
    Real(RealNumber),
}

#[derive(Clone, Debug)]
pub struct Number {
    pub(crate) kind: NumberKind,
}

impl Number {
    pub fn get_nan() -> Self {
        Self {
            kind: NumberKind::NaN,
        }
    }

    pub(crate) fn get_real(real_number: RealNumber) -> Self {
        Self {
            kind: NumberKind::Real(real_number),
        }
    }

    pub fn from_number_node(number_node: NumNode) -> Self {
        match RealNumber::from_unsigned_numeric_string(number_node.val()) {
            Some(real_number) => Self::get_real(real_number),
            _ => Self::get_nan(),
        }
    }

    pub fn is_nan(&self) -> bool {
        if let NumberKind::NaN = self.kind {
            true
        } else {
            false
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            NumberKind::NaN => write!(f, "NaN"),
            NumberKind::Real(real_number) => write!(f, "{}", real_number.to_string()),
        }
    }
}
