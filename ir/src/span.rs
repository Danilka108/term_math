use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn concat(&self, other: &Self) -> Self {
        Self::new(self.start, other.end)
    }
}

#[derive(Debug, Clone)]
pub struct SpanWrapper<V>(V, Span) where V: Debug + Clone;

impl<V> SpanWrapper<V> where V: Debug + Clone {
    pub fn new(val: V, span: Span) -> Self {
        Self(val, span)
    }

    pub fn borrow_val(&self) -> &V {
        &self.0
    }

    pub fn val(self) -> V {
        self.0
    }

    pub fn span(&self) -> &Span {
        &self.1
    }
}
