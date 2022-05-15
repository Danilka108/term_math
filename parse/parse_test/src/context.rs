use ir::span::{SpanWrapper, Span};
use std::fmt::Debug;
use std::marker::PhantomData;

pub type PResult<D = ()> = Result<D, SpanWrapper<String>>;

pub trait Tokenizer<T: Clone + Debug>: Clone + Debug {
    fn curr(&self) -> SpanWrapper<T>;
    fn first(&self) -> SpanWrapper<T>;
    fn second(&self) -> SpanWrapper<T>;
    fn is_eof(&self) -> bool;
    fn bump(&mut self);
}

pub trait Stack<V: Clone + Debug>: Clone + Debug {
    fn pop(&mut self) -> Option<SpanWrapper<V>>;
    fn pop_tuple(&mut self) -> Option<(V, Span)>;

    fn push(&mut self, item: SpanWrapper<V>);
    fn push_tuple(&mut self, val: V, span: Span);
}

#[derive(Clone, Debug)]
pub struct Context<T, B, N, TK, BS, NS>
where
    T: Clone + Debug,
    N: Clone + Debug,
    B: Clone + Debug,
    TK: Tokenizer<T>,
    NS: Stack<N>,
    BS: Stack<B>,
{
    pub tokenizer: TK,
    pub nodes: NS,
    pub buffer: BS,
    _t: PhantomData<T>,
    _b: PhantomData<B>,
    _n: PhantomData<N>,
}

impl<T, B, N, TK, BS, NS> Context<T, B, N, TK, BS, NS>
where
    T: Clone + Debug,
    N: Clone + Debug,
    B: Clone + Debug,
    TK: Tokenizer<T>,
    NS: Stack<N>,
    BS: Stack<B>,
{
    pub fn new(tokenizer: TK, buffer: BS, nodes: NS) -> Self {
        Self {
            tokenizer,
            buffer,
            nodes,
            _t: PhantomData,
            _b: PhantomData,
            _n: PhantomData,
        }
    }
}

pub trait ParserHooks {
    fn parsing_hook(&mut self) -> PResult;

    fn eof_hook(&mut self) -> PResult;
}
