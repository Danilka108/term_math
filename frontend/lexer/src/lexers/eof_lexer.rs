use crate::lexer::Lexer;
use token::{Token, TokenKind, TokenSpan};

impl<'s> Lexer<'s> {
    pub(crate) fn lex_eof(&mut self) -> Option<Token> {
        if !self.symbol_stream.is_eof() {
            return None;
        }

        let pos = self.symbol_stream.eof_pos();
        let span = TokenSpan::new(pos, pos + 1);
        let kind = TokenKind::Eof;

        Some(Token::new(kind, span))
    }
}
