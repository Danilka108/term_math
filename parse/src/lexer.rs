use ir::span::*;
use ir::token::*;
use std::fmt::Debug;
use std::str::Chars;

const EOF_CHAR: char = '\0';

pub fn is_lit(chr: char) -> bool {
    chr == '*' || chr == '/' || chr == '+' || chr == '-' || chr == ','
}

pub fn is_delim(chr: char) -> bool {
    chr == '(' || chr == ')' || chr == '{' || chr == '}' || chr == '[' || chr == ']'
}

pub fn is_number_start(chr: char) -> bool {
    chr.is_digit(10)
}

pub fn is_number(chr: char) -> bool {
    chr.is_digit(10) || chr.is_ascii_alphabetic() || chr == '.'
}

pub fn is_ident_start(chr: char) -> bool {
    chr.is_alphabetic()
}

pub fn is_ident(chr: char) -> bool {
    chr.is_alphabetic() || chr.is_digit(10)
}

pub fn is_whitespace(chr: char) -> bool {
    matches!(
        chr,
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

pub fn is_unknown(chr: char) -> bool {
    !(is_number_start(chr)
        || is_number(chr)
        || is_ident_start(chr)
        || is_ident(chr)
        || is_lit(chr)
        || is_delim(chr)
        || is_whitespace(chr))
}

pub struct Lexer<'chars> {
    chars: Chars<'chars>,
    offset: usize,
}

impl<'chars> Lexer<'chars> {
    const PRODUCERS: [fn(&mut Self) -> Option<SpanWrapper<Token>>; 6] = [
        Self::produce_lit,
        Self::produce_delim,
        Self::produce_ident,
        Self::produce_num,
        Self::produce_whitespace,
        Self::produce_unknown,
    ];

    pub fn new(expr: &'chars str) -> Self {
        Self {
            chars: expr.chars(),
            offset: 0,
        }
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn bump(&mut self) {
        self.offset += 1;
        self.chars.next();
    }

    fn first(&mut self) -> SpanWrapper<char> {
        let val = self.chars.clone().next().unwrap_or(EOF_CHAR);
        let span = Span::new(self.offset, self.offset + 1);
        SpanWrapper::new(val, span)
    }

    fn cut<V: Clone + Debug>(&mut self, wrapper: &SpanWrapper<V>) {
        let end = wrapper.borrow_span().clone().end();

        while self.offset < end {
            self.bump();
        }
    }

    fn consume_while(
        &mut self,
        mut start_predicate: impl FnMut(char) -> bool,
        mut tail_predicate: impl FnMut(char) -> bool,
    ) -> Option<SpanWrapper<String>> {
        if !start_predicate(self.first().val()) {
            return None;
        }

        let (start_val, start_span) = self.first().to_tuple();
        self.bump();

        let tail_val = self
            .chars
            .clone()
            .take_while(|chr| tail_predicate(chr.clone()))
            .collect::<String>();
        let tail_span = Span::new(self.offset, self.offset + tail_val.len());

        for _ in 0..tail_val.len() {
            self.bump();
        }

        let val = [String::from(start_val), tail_val].concat();
        let span = [start_span, tail_span].concat_span();

        Some(SpanWrapper::new(val, span))
    }

    fn produce_lit(&mut self) -> Option<SpanWrapper<Token>> {
        let lit_kind = match self.first().val() {
            '*' => LitKind::Asterisk,
            '/' => LitKind::Slash,
            '+' => LitKind::Plus,
            '-' => LitKind::Hyphen,
            ',' => LitKind::Comma,
            _ => return None,
        };

        Some(SpanWrapper::new(Token::Lit(lit_kind), self.first().span()))
    }

    fn produce_delim(&mut self) -> Option<SpanWrapper<Token>> {
        let delim_kind = match self.first().val() {
            '(' => Token::OpenDelim(DelimKind::Paren),
            ')' => Token::CloseDelim(DelimKind::Paren),
            '{' => Token::OpenDelim(DelimKind::Brace),
            '}' => Token::CloseDelim(DelimKind::Brace),
            '[' => Token::OpenDelim(DelimKind::Bracket),
            ']' => Token::CloseDelim(DelimKind::Bracket),
            _ => return None,
        };

        Some(SpanWrapper::new(delim_kind, self.first().span()))
    }

    fn produce_ident(&mut self) -> Option<SpanWrapper<Token>> {
        self.consume_while(is_ident_start, is_ident)
            .map(|w| w.map(|v| Token::Ident(v)))
    }

    fn produce_num(&mut self) -> Option<SpanWrapper<Token>> {
        self.consume_while(is_number_start, is_number)
            .map(|w| w.map(|v| Token::Num(v)))
    }

    fn produce_whitespace(&mut self) -> Option<SpanWrapper<Token>> {
        self.consume_while(is_whitespace, is_whitespace)
            .map(|w| w.map(|_| Token::Whitespace))
    }

    fn produce_unknown(&mut self) -> Option<SpanWrapper<Token>> {
        self.consume_while(is_unknown, is_unknown)
            .map(|w| w.map(|_| Token::Unknown))
    }

    pub fn tokenize(mut self) -> TokenStream {
        let mut tokens = Vec::new();

        'l: while !self.is_eof() {
            for produce in Self::PRODUCERS {
                if let Some(token) = produce(&mut self) {
                    self.cut(&token);
                    tokens.push(token);
                    continue 'l;
                }
            }
        }

        tokens.push(self.first().map(|_| Token::Eof));

        tokens.into_iter()
    }
}
