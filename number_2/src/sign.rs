use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum Sign {
    Neg,
    Pos,
}

impl PartialEq for Sign {
    fn eq(&self, rhs: &Self) -> bool {
        matches!(self.cmp(rhs), Ordering::Equal)
    }
}

impl Eq for Sign {}

impl PartialOrd for Sign {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Sign {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Self::Pos, Self::Neg) => Ordering::Greater,
            (Self::Neg, Self::Pos) => Ordering::Less,
            _ => Ordering::Equal
        }
    }
}

impl Sign {
    pub fn is_pos(&self) -> bool {
        matches!(&self, Self::Pos)
    }

    pub fn is_neg(&self) -> bool {
        matches!(&self, Self::Neg)
    }
    
    pub fn to_char(&self) -> char {
        match self {
            Self::Pos => '+',
            Self::Neg => '-',
        }
    }
}
