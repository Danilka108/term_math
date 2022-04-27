#[derive(Clone, Debug)]
pub enum Sign {
    Neg,
    Pos,
}

impl Sign {
    pub fn reverse(self) -> Self {
        match self {
            Self::Pos => Self::Neg,
            Self::Neg => Self::Pos,
        }
    }
}
