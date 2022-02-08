use crate::lexer::Lexer;
use ast::token::{Token, TokenKind};
use crate::constants::ERR__INVALID_NUMBER_VALUE;

impl<'s> Lexer<'s> {
    fn is_number_valid(val: &String) -> bool {
        let val_parts = val.split(".").collect::<Vec<_>>();
        val_parts.len() == 2 && val_parts[1].len() != 0 || val_parts.len() == 1
    }

    pub(crate) fn lex_number(&mut self) -> Option<Token> {
        let is_digit = |sym: char| sym.is_digit(10) || sym == '.';
        let (span, val) = self.lex_while(is_digit)?;

        let kind = if Self::is_number_valid(&val) {
            TokenKind::Number(val)
        } else {
            TokenKind::Error(ERR__INVALID_NUMBER_VALUE.to_string())
        };

        Some(Token::new(kind, span))
    }
}
