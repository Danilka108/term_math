use std::iter::{Enumerate, Peekable};
use std::str::Chars;

#[derive(Clone, Debug)]
pub(crate) struct SymbolStream<'c> {
    chars: Peekable<Enumerate<Chars<'c>>>,
    initial_len: usize,
}

impl<'c> SymbolStream<'c> {
    pub(crate) fn new(input: &'c String) -> Self {
        Self {
            initial_len: input.len(),
            chars: input.chars().enumerate().peekable(),
        }
    }

    fn len(&mut self) -> usize {
        self.chars
            .clone()
            .map(|(_, sym)| sym)
            .collect::<String>()
            .len()
    }

    pub(crate) fn to_next(&mut self) -> Option<char> {
        self.chars.next().map(|(_, sym)| sym)
    }

    pub(crate) fn next(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, sym)| sym.clone())
    }

    pub(crate) fn pos(&mut self) -> Option<usize> {
        let pos = self.initial_len - self.len();

        if pos == 0 {
            None
        } else {
            Some(pos - 1)
        }
    }

    pub(crate) fn is_eof(&mut self) -> bool {
        self.len() == 0
    }

    pub(crate) fn eof_pos(&mut self) -> usize {
        self.initial_len
    }
}
