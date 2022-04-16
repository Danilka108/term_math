use ir::attempt::*;
use ir::hl_token::{HlToken, HlTokenKind};
use ir::ll_token::{LlToken, LlTokenKind, LlTokenStream};
use ir::span::Span;

#[derive(Clone, Debug)]
pub struct Cursor {
    curr: LlTokenKind,
    ll_stream: LlTokenStream,
}

impl Cursor {
    pub fn new(ll_stream: LlTokenStream) -> Self {
        Self {
            curr: LlTokenKind::Eof,
            ll_stream,
        }
    }

    pub fn next(&self) -> LlTokenKind {
        self.ll_stream
            .clone()
            .next()
            .map(|t| t.val())
            .unwrap_or(LlTokenKind::Eof)
    }

    fn offset(&self) -> usize {
        self.ll_stream
            .clone()
            .next()
            .map(|t| t.span().start())
            .unwrap_or(0)
    }

    fn len(&self) -> usize {
        self.ll_stream
            .clone()
            .last()
            .map(|t| t.span().end())
            .unwrap_or(self.offset())
    }

    pub fn span(&self) -> Span {
        Span::new(self.offset(), self.len())
    }

    pub fn bump(&mut self) -> LlToken {
        let eof = LlToken::new(LlTokenKind::Eof, Span::new(self.offset(), self.len()));
        let next = self.ll_stream.next().unwrap_or(eof);
        self.curr = next.clone().val();
        next
    }

    pub fn is_eof(&self) -> bool {
        self.next() == LlTokenKind::Eof
    }

    pub fn as_ll_stream(self) -> LlTokenStream {
        self.ll_stream
    }
}

impl AttemptNext<Cursor> for Cursor {
    type Item = LlTokenKind;

    fn attempt_next(self, mut predicate: impl FnMut(Self::Item) -> bool) -> Attempt<Cursor> {
        if predicate(self.next()) {
            Attempt::Succeeded(self)
        } else {
            Attempt::Failed
        }
    }
}

impl AttemptCurr<Cursor> for Cursor {
    type Item = LlTokenKind;

    fn attempt_curr(self, mut predicate: impl FnMut(Self::Item) -> bool) -> Attempt<Cursor> {
        if predicate(self.curr.clone()) {
            Attempt::Succeeded(self)
        } else {
            Attempt::Failed
        }
    }
}

impl ConsumeNext<Cursor> for Cursor {
    type Item = LlTokenKind;

    fn consume_next(mut self, mut predicate: impl FnMut(Self::Item) -> bool) -> Attempt<Cursor> {
        if !predicate(self.next()) && !self.is_eof() {
            return Attempt::Failed;
        }

        let tokens = vec![self.bump()];
        Attempt::Succeeded(Self::new(tokens.into_iter()))
    }
}

impl ConsumeWhile<Cursor> for Cursor {
    type Item = LlTokenKind;

    fn consume_while(mut self, mut predicate: impl FnMut(Self::Item) -> bool) -> Attempt<Cursor> {
        let mut tokens = Vec::new();

        while predicate(self.next()) && !self.is_eof() {
            tokens.push(self.bump());
        }

        Attempt::Succeeded(Self::new(tokens.into_iter()))
    }
}

impl MapToToken<Cursor> for Cursor {
    type TokenKind = Result<HlTokenKind, String>;
    type Token = Result<HlToken, String>;

    fn map_to_token(
        self,
        mut predicate: impl FnMut(Cursor) -> Option<Self::TokenKind>,
    ) -> Option<Self::Token> {
        let span = self.span();
        Some(predicate(self)?.map(|kind| HlToken::new(kind, span)))
    }
}
