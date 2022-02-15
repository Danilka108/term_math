use token::TokenSpan;

pub(crate) struct Combiner {
    val: String,
    span: Option<TokenSpan>,
}

impl Combiner {
    pub(crate) fn new() -> Self {
        Self {
            val: String::new(),
            span: None,
        }
    }

    pub(crate) fn push(&mut self, sym: char, pos: usize) {
        self.span = match self.span.clone() {
            None => Some(TokenSpan::new(pos, pos + 1)),
            Some(span) => Some(TokenSpan::new(span.start(), pos + 1)),
        };

        self.val.push(sym);
    }

    pub(crate) fn combine(&mut self) -> Option<(TokenSpan, String)> {
        if self.val.len() == 0 {
            return None;
        }

        let res = Some((self.span.clone()?, self.val.clone()));

        self.val.clear();
        self.span = None;

        res
    }
}
