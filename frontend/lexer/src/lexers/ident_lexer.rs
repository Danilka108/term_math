use crate::lexer::Lexer;
use token::{Token, TokenKind};

impl<'s> Lexer<'s> {
    pub(crate) fn lex_ident(&mut self) -> Option<Token> {
        let is_alphabetic = |sym: char| sym.is_alphabetic();

        let (span, val) = self.lex_while(is_alphabetic)?;
        let kind = TokenKind::Ident(val);

        Some(Token::new(kind, span))
    }
}
