use crate::lexer::Lexer;
use token::{LiteralKind, Token, TokenKind};

impl<'s> Lexer<'s> {
    pub(crate) fn lex_literal(&mut self) -> Option<Token> {
        let identify = |sym| match sym {
            '*' => Some(TokenKind::Literal(LiteralKind::Asterisk)),
            '/' => Some(TokenKind::Literal(LiteralKind::Slash)),
            '-' => Some(TokenKind::Literal(LiteralKind::Hyphen)),
            '+' => Some(TokenKind::Literal(LiteralKind::Plus)),
            ',' => Some(TokenKind::Literal(LiteralKind::Comma)),
            _ => None,
        };

        self.lex_char(identify)
    }
}
