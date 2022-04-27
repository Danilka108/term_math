use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Modification<OK: Clone + Debug, ERR: Clone + Debug> {
    Ok(OK),
    Err(ERR),
    None,
}

impl<OK: Clone + Debug, ERR: Clone + Debug> Modification<OK, ERR> {
    pub fn map<N: Clone + Debug>(self, mut predicate: impl FnMut(OK) -> N) -> Modification<N, ERR> {
        match self {
            Modification::Ok(val) => Modification::Ok(predicate(val)),
            Modification::Err(err) => Modification::Err(err),
            Modification::None => Modification::None,
        }
    }

    pub fn nonerr(self) -> Self {
        match self {
            Self::Err(_) => Self::None,
            other => other,
        }
    }

    pub fn or(self, other: Modification<OK, ERR>) -> Modification<OK, ERR> {
        match (self, other) {
            (Modification::Ok(ok), _) | (_, Modification::Ok(ok)) => Modification::Ok(ok),
            (Modification::Err(err), _) => Modification::Err(err),
            (_, right) => right,
        }
    }

    pub fn unwrap_as_option(self) -> Option<OK> {
        match self {
            Self::Ok(ok) => Some(ok),
            Self::None => None,
            Self::Err(_) => panic!("called 'Modify::unwrap_as_option' on a 'Modify::Err' value"),
        }
    }
}
