use crate::lexer::Lexer;
use token::{TokenKind, DelimKind, Token};

impl<'s> Lexer<'s> {
    pub(crate) fn lex_delim(&mut self) -> Option<Token> {
        let identify = |sym| match sym {
            '(' => Some(TokenKind::OpenDelim(DelimKind::Paren)),
            ')' => Some(TokenKind::CloseDelim(DelimKind::Paren)),
            '{' => Some(TokenKind::OpenDelim(DelimKind::Brace)),
            '}' => Some(TokenKind::CloseDelim(DelimKind::Brace)),
            ']' => Some(TokenKind::OpenDelim(DelimKind::Bracket)),
            '[' => Some(TokenKind::CloseDelim(DelimKind::Bracket)),
            _ => None,
        };


        self.lex_char(identify)
    }
}
