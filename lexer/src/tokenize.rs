use crate::cursor::*;
use crate::token::*;
use ast::span::Span;

const ERR__UNKNOWN_SYMBOLS: &str = "Unknown symbols";

pub struct Lexer<'a> {
    unknown_token_span: Span,
    cursor: Cursor<'a>,
    tokens: Vec<Token<'a>>,
}

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
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

fn consume_lit<'c>(cursor: Cursor<'c>) -> Option<Token<'c>> {
    let is_lit = |chr: char| chr == '*' || chr == '/' || chr == '+' || chr == '-' || chr == ',';

    cursor
        .attempt_next(is_lit)
        .consume_next(is_lit)
        .map_to_token(lit_from_char)
}

fn consume_delim<'c>(cursor: Cursor<'c>) -> Option<Token<'c>> {
    let is_delim = |chr: char| {
        chr == '(' || chr == ')' || chr == '{' || chr == '}' || chr == '[' || chr == ']'
    };

    cursor
        .attempt_next(is_delim)
        .consume_next(is_delim)
        .map_to_token(delim_from_char)
}

fn consume_number<'c>(cursor: Cursor<'c>) -> Option<Token<'c>> {
    let is_digit = |chr: char| chr.is_digit(10) || chr == '.';

    cursor
        .attempt_next(is_digit)
        .consume_while(is_digit)
        .map_to_token(num_from_str)
}

fn consume_ident<'c>(cursor: Cursor<'c>) -> Option<Token<'c>> {
    let is_ident_start = |chr: char| chr.is_alphabetic();
    let is_ident_tail = |chr: char| chr.is_alphabetic() || chr.is_digit(10);

    cursor
        .attempt_next(is_ident_start)
        .consume_while(is_ident_tail)
        .map_to_token(ident_from_str)
}

impl<'s> Lexer<'s> {
    const CONSUMERS: [for<'c> fn(Cursor<'c>) -> Option<Token<'c>>; 4] =
        [consume_number, consume_ident, consume_lit, consume_delim];

    pub fn new(src: &'s str) -> Self {
        Self {
            cursor: Cursor::new(src),
            tokens: Vec::new(),
            unknown_token_span: Span::new(0, 0),
        }
    }

    fn push_unknown_token(&mut self) {
        if self.unknown_token_span.start() > self.unknown_token_span.end() {
            let kind = TokenKind::Err(ERR__UNKNOWN_SYMBOLS.to_string());
            self.tokens
                .push(Token::new(kind, self.unknown_token_span.clone()));
            self.unknown_token_span = Span::new(self.cursor.pos(), self.cursor.pos());
        }
    }

    fn increment_unknown_token_span(&mut self) {
        self.unknown_token_span = Span::new(
            self.unknown_token_span.start(),
            self.unknown_token_span.end() + 1,
        );
    }

    fn push_eof_token(&mut self) {
        let span = Span::new(self.cursor.len(), self.cursor.len() + 1);
        self.tokens.push(Token::new(TokenKind::Eof, span));
    }

    fn consume_token(&mut self) {
        let consumed_token = Self::CONSUMERS
            .iter()
            .find_map(|consume| consume(self.cursor.clone()));

        let token = if let Some(token) = consumed_token {
            token
        } else {
            self.increment_unknown_token_span();
            return;
        };

        self.push_unknown_token();
        self.cursor.eat_token(&token);
        self.tokens.push(token);
    }

    pub fn tokenize(mut self) -> impl Iterator<Item = Token<'s>> {
        while !self.cursor.is_eof() {
            self.consume_token();
            self.cursor.eat_while(is_whitespace);
        }

        self.push_eof_token();
        self.tokens.into_iter()
    }
}
