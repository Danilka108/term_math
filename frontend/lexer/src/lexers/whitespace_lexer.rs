use crate::lexer::Lexer;
use token::Token;

impl<'s> Lexer<'s> {
    fn is_whitespace(sym: char) -> bool {
        matches!(
            sym,
            // Usual ASCII suspects
            '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
        )
    }

    pub(crate) fn lex_whitespace(&mut self) -> Option<Token> {
        self.lex_while(Self::is_whitespace);
        None
    }
}
