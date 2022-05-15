use crate::SpanWrapper;
use std::any::Any;
use std::marker::PhantomData;
use std::rc::Rc;

pub trait Cursor<T> {
    fn curr(&self) -> SpanWrapper<T>;
    fn first(&self) -> SpanWrapper<T>;
    fn second(&self) -> SpanWrapper<T>;
    fn is_eof(&self) -> bool;
    fn bump(&mut self);
}

#[derive(Clone)]
pub struct Tokenizer<T> {
    cursor: Rc<Box<dyn Cursor<T>>>,
}

impl<T> Tokenizer<T> {
    pub fn new(cursor: Box<dyn Cursor<T>>) -> Self {
        Self {
            cursor: Rc::new(cursor),
        }
    }
}

impl<T> Cursor<T> for Tokenizer<T> {
    fn curr(&self) -> SpanWrapper<T> {
        self.cursor.curr()
    }

    fn first(&self) -> SpanWrapper<T> {
        self.cursor.first()
    }

    fn second(&self) -> SpanWrapper<T> {
        self.cursor.second()
    }

    fn is_eof(&self) -> bool {
        self.cursor.is_eof()
    }

    fn bump(&mut self) {
        if let Some(cursor) = Rc::get_mut(&mut self.cursor) {
            cursor.bump()
        }
    }
}

#[derive(Clone)]
pub struct Buffer<V> {
    stack: Rc<Vec<Box<dyn Any>>>,
    _phantom: PhantomData<V>,
}

impl<V> Buffer<V> {
    pub fn change_val_ty<I>(self) -> Buffer<I> {
        Buffer {
            stack: self.stack,
            _phantom: PhantomData,
        }
    }
}

impl<V> Default for Buffer<V> {
    fn default() -> Self {
        Self {
            stack: Rc::new(Vec::new()),
            _phantom: PhantomData,
        }
    }
}

impl<V: 'static> Buffer<V> {
    pub fn pop(&mut self) -> Option<V> {
        match Rc::get_mut(&mut self.stack)?.pop() {
            Some(val) => match val.downcast() {
                Ok(val) => Some(*val),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn push(&mut self, val: V) {
        match Rc::get_mut(&mut self.stack) {
            Some(stack) => stack.push(Box::new(val)),
            _ => (),
        }
    }
}

#[derive(Clone)]
pub struct Nodes<N> {
    stack: Rc<Vec<N>>,
}

impl<N> Default for Nodes<N> {
    fn default() -> Self {
        Self {
            stack: Rc::new(Vec::new()),
        }
    }
}

impl<N> Nodes<N> {
    pub fn pop(&mut self) -> Option<N> {
        Rc::get_mut(&mut self.stack)?.pop()
    }

    pub fn push(&mut self, val: N) {
        match Rc::get_mut(&mut self.stack) {
            Some(stack) => stack.push(val),
            _ => (),
        }
    }
}

pub struct Context<T, B, N> {
    pub tokenizer: Tokenizer<T>,
    pub buffer: Buffer<B>,
    pub nodes: Nodes<N>,
}
