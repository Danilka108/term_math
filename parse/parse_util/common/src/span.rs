#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn decrement_start(mut self) -> Self {
        self.start = self.start.checked_sub(1).unwrap_or(0);
        self
    }

    pub fn increment_end(mut self) -> Self {
        self.end += 1;
        self
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

pub trait ConcatSpan {
    fn concat_span(self) -> Span;
}

impl ConcatSpan for [&Span; 2] {
    fn concat_span(self) -> Span {
        let start = self[0].start().min(self[1].start());
        let end = self[0].end().max(self[1].end());
        Span::new(start, end)
    }
}

impl ConcatSpan for [Span; 2] {
    fn concat_span(self) -> Span {
        let start = self[0].start().min(self[1].start());
        let end = self[0].end().max(self[1].end());
        Span::new(start, end)
    }
}

#[derive(Clone)]
pub struct SpanWrapper<V>(V, Span);

impl<V> SpanWrapper<V>
{
    pub fn new(val: V, span: Span) -> Self {
        Self(val, span)
    }

    pub fn to_tuple(self) -> (V, Span) {
        (self.0, self.1)
    }

    pub fn mut_borrow_to_tuple(&mut self) -> (&mut V, &mut Span) {
        (&mut self.0, &mut self.1)
    }

    pub fn borrow_to_tuple(&self) -> (&V, &Span) {
        (&self.0, &self.1)
    }

    pub fn map<N: Clone, FN: FnMut(V) -> N>(self, mut f: FN) -> SpanWrapper<N> {
        let SpanWrapper (val, span) = self;
        SpanWrapper(f(val), span)
    }

    pub fn val(self) -> V {
        self.0
    }

    pub fn borrow_val(&self) -> &V {
        &self.0
    }

    pub fn mut_borrow_val(&mut self) -> &mut V {
        &mut self.0
    }

    pub fn span(self) -> Span {
        self.1
    }

    pub fn borrow_span(&self) -> &Span {
        &self.1
    }

    pub fn mut_borrow_span(&mut self) -> &mut Span {
        &mut self.1
    }
}
