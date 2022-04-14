use crate::token::{Token, TokenKind};
use ast::span::Span;
use std::str::Chars;

const EOF_CHAR: char = '\0';

pub trait AttemptNext<'c> {
    fn attempt_next(self, predicate: impl FnMut(char) -> bool) -> Attempt<'c>;
}

pub trait ConsumeNext {
    fn consume_next(self, predicate: impl FnMut(char) -> bool) -> Self;
}

pub trait ConsumeWhile {
    fn consume_while(self, predicate: impl FnMut(char) -> bool) -> Self;
}

pub trait MapToToken<'c> {
    fn map_to_token(
        self,
        predicate: impl FnMut(&'c str) -> Option<TokenKind<'c>>,
    ) -> Option<Token<'c>>;
}

pub enum Attempt<'c> {
    Succeeded(Cursor<'c>),
    Failed,
}

#[derive(Debug, Clone)]
pub struct Cursor<'c> {
    chars: Chars<'c>,
    offset: usize,
}

impl<'c> Cursor<'c> {
    pub fn new(src: &'c str) -> Self {
        Self {
            chars: src.chars(),
            offset: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.offset
    }

    pub fn len(&self) -> usize {
        self.chars.clone().count() + self.offset
    }

    fn get_span(&self) -> Span {
        Span::new(self.pos(), self.pos() + self.chars.clone().count())
    }

    fn eat_span(&mut self, span: &Span) {
        while self.offset < span.end() {
            self.bump();
        }
    }

    pub fn eat_token(&mut self, token: &Token<'c>) {
        self.eat_span(&token.span);
    }

    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> Chars<'c> {
        let old = self.clone();

        while predicate(self.next()) && !self.is_eof() {
            self.bump();
        }

        old.chars.as_str()[..(self.offset - old.offset)].chars()
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn bump(&mut self) -> Option<char> {
        self.offset += 1;
        let next_char = self.chars.next();

        next_char
    }

    fn next(&self) -> char {
        self.clone().bump().unwrap_or(EOF_CHAR)
    }

    pub fn as_str(self) -> &'c str {
        self.chars.as_str()
    }
}

impl<'c> AttemptNext<'c> for Cursor<'c> {
    fn attempt_next(self, predicate: impl FnOnce(char) -> bool) -> Attempt<'c> {
        if predicate(self.next()) {
            Attempt::Succeeded(self)
        } else {
            Attempt::Failed
        }
    }
}

impl<'c> ConsumeNext for Cursor<'c> {
    fn consume_next(mut self, mut predicate: impl FnMut(char) -> bool) -> Self {
        let range = if predicate(self.next()) { 0..1 } else { 0..0 };

        self.chars = self.clone().chars.as_str()[range].chars();

        self
    }
}

impl<'c> ConsumeWhile for Cursor<'c> {
    fn consume_while(mut self, predicate: impl FnMut(char) -> bool) -> Self {
        let mut old = self.clone();
        old.chars = self.eat_while(predicate);
        old
    }
}

impl<'c> MapToToken<'c> for Cursor<'c> {
    fn map_to_token(
        self,
        mut predicate: impl FnMut(&'c str) -> Option<TokenKind<'c>>,
    ) -> Option<Token<'c>> {
        let span = self.get_span();
        let val = self.as_str();
        let kind = predicate(val)?;

        Some(Token::new(kind, span))
    }
}

impl<'c> AttemptNext<'c> for Attempt<'c> {
    fn attempt_next(self, predicate: impl FnMut(char) -> bool) -> Attempt<'c> {
        match self {
            Self::Succeeded(cursor) => cursor.attempt_next(predicate),
            f => f,
        }
    }
}

impl<'c> ConsumeNext for Attempt<'c> {
    fn consume_next(self, predicate: impl FnMut(char) -> bool) -> Self {
        match self {
            Self::Succeeded(cursor) => Self::Succeeded(cursor.consume_next(predicate)),
            f => f,
        }
    }
}

impl<'c> ConsumeWhile for Attempt<'c> {
    fn consume_while(self, predicate: impl FnMut(char) -> bool) -> Attempt<'c> {
        match self {
            Self::Succeeded(cursor) => Attempt::Succeeded(cursor.consume_while(predicate)),
            f => f,
        }
    }
}

impl<'c> MapToToken<'c> for Attempt<'c> {
    fn map_to_token(
        self,
        predicate: impl FnMut(&'c str) -> Option<TokenKind<'c>>,
    ) -> Option<Token<'c>> {
        match self {
            Self::Succeeded(cursor) => cursor.map_to_token(predicate),
            _ => None,
        }
    }
}
