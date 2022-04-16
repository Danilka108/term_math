use ir::cursor::*;
use ir::token::Token;
use std::str::Chars;

const EOF_CHAR: char = '\0';

#[derive(Debug, Clone)]
pub struct StringCursor<'c> {
    curr: &'c char,
    chars: Chars<'c>,
    offset: usize,
}

impl<'c> StringCursor<'c> {
    pub fn new(val: &'c str) -> Self {
        Self {
            chars: val.chars(),
            curr: &EOF_CHAR,
            offset: 0,
        }
    }
}


impl<'c> Cursor<'c, Token> for StringCursor<'c> {
    type Item = &'c char;

    fn bump(&mut self) -> Option<Self::Item> {
        let next_char = self.chars.next();

        self.offset += 1;
        self.curr = next_char.unwrap_or(&EOF_CHAR);

        next_char
    }

    fn curr(&self) -> Self::Item {
        self.curr
    }

    fn start(&self) -> usize {
        self.offset
    }

    fn end(&self) -> usize {
        self.offset + self.chars.clone().count()
    }
}
