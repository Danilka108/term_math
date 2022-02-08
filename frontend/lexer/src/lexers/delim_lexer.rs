use crate::lexer::Lexer;
use ast::token::{TokenKind, DelimToken, Token};

impl<'s> Lexer<'s> {
    pub(crate) fn lex_delim(&mut self) -> Option<Token> {
        let identify = |sym| match sym {
            '(' => Some(TokenKind::OpenDelim(DelimToken::Paren)),
            ')' => Some(TokenKind::CloseDelim(DelimToken::Paren)),
            '{' => Some(TokenKind::OpenDelim(DelimToken::Brace)),
            '}' => Some(TokenKind::CloseDelim(DelimToken::Brace)),
            ']' => Some(TokenKind::OpenDelim(DelimToken::Bracket)),
            '[' => Some(TokenKind::CloseDelim(DelimToken::Bracket)),
            _ => None,
        };

        self.lex_char(identify)
    }
}
