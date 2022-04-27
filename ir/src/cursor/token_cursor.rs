use super::cursor::{Cursor, FromCursor, IntoCursor};
use crate::span::{SharedSpanWrapper, Span, SpanWrapper};
use crate::token::{SharedTokenStream, Token};

#[derive(Clone, Debug)]
pub struct TokenCursor<'t> {
    tokens: SharedTokenStream<'t>,
    curr_token: SpanWrapper<&'t Token>,
}

impl<'t> Cursor for TokenCursor<'t> {
    type Item = &'t Token;
    type Original = Self;

    fn bump(&mut self) -> Option<SpanWrapper<Self::Item>> {
        match self.tokens.next() {
            Some(token) => {
                let next = SharedSpanWrapper::from(token);
                self.curr_token = next.clone();
                Some(next)
            },
            None => {
                self.curr_token = SpanWrapper::new(&Token::Eof, self.curr_token.borrow_span().clone());
                None
            }
        }
    }

    fn curr(&self) -> SpanWrapper<Self::Item> {
        self.curr_token.clone()
    }

    fn original(&self) -> &Self::Original {
        &self
    }
}

impl<'t> IntoCursor for SharedTokenStream<'t> {
    type Item = &'t Token;
    type IntoCursor = TokenCursor<'t>;

    fn into_cursor(self) -> Self::IntoCursor {
        let tokens = self;
        let curr_token = SpanWrapper::new(&Token::Eof, Span::new(0, 0));

        TokenCursor { tokens, curr_token }
    }
}

impl<'t> FromCursor<&'t Token> for SharedTokenStream<'t> {
    fn from_cursor<CURSOR: IntoCursor<Item = &'t Token>>(c: CURSOR) -> Self {
        let mut cursor = c.into_cursor();
        let mut tokens = Vec::new();

        while let Some(token) = cursor.bump() {
            tokens.push(token);
        }

        tokens.into_iter()
    }
}
