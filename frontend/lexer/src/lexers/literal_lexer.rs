use crate::lexer::Lexer;
use token::{LiteralToken, Token, TokenKind};

impl<'s> Lexer<'s> {
    pub(crate) fn lex_literal(&mut self) -> Option<Token> {
        let identify = |sym| match sym {
            '*' => Some(TokenKind::Literal(LiteralToken::Asterisk)),
            '/' => Some(TokenKind::Literal(LiteralToken::Slash)),
            '-' => Some(TokenKind::Literal(LiteralToken::Hyphen)),
            '+' => Some(TokenKind::Literal(LiteralToken::Plus)),
            ',' => Some(TokenKind::Literal(LiteralToken::Comma)),
            _ => None,
        };

        self.lex_char(identify)
    }
}
