/*
mod codegen;
mod context;
mod event;
*/
mod codegen_2;

/*
use ir::ast::*;
use ir::span::*;
use ir::token::*;
use std::fmt::Debug;

use crate::context::Tokenizer; use ir::span::*;
use ir::token::*;
use std::str::Chars;


const EOF_CHAR: char = '\0';

pub fn test(expr: &'static str) -> context::PResult<Box<SpanWrapper<Node>>> {
    let tokenizer = MathTokenizer {
        pos: 0,
        chars: expr.chars(),
        curr: SpanWrapper::new(Token::Eof, Span::new(0, 0)),
    };

    codegen::parse(tokenizer)
}

fn is_whitespace(chr: char) -> bool {
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

fn produce_single_len_token(chr: char) -> Option<Token> {
    let token = match chr {
        '*' => Token::Lit(LitKind::Asterisk),
        '/' => Token::Lit(LitKind::Slash),
        '+' => Token::Lit(LitKind::Plus),
        '-' => Token::Lit(LitKind::Hyphen),
        ',' => Token::Lit(LitKind::Comma),
        '(' => Token::OpenDelim(DelimKind::Paren),
        ')' => Token::CloseDelim(DelimKind::Paren),
        '{' => Token::OpenDelim(DelimKind::Brace),
        '}' => Token::CloseDelim(DelimKind::Brace),
        '[' => Token::OpenDelim(DelimKind::Bracket),
        ']' => Token::CloseDelim(DelimKind::Bracket),
        _ => return None,
    };

    Some(token)
}

pub fn is_number_start(chr: char) -> bool {
    chr.is_digit(10)
}

pub fn is_number_tail(chr: char) -> bool {
    chr.is_digit(10) || chr.is_ascii_alphabetic() || chr == '.'
}

pub fn is_ident_start(chr: char) -> bool {
    chr.is_alphabetic()
}

pub fn is_ident_tail(chr: char) -> bool {
    chr.is_alphabetic() || chr.is_digit(10)
}

pub fn is_unknown(chr: char) -> bool {
    !(is_number_start(chr)
        || is_number_tail(chr)
        || is_ident_start(chr)
        || is_ident_tail(chr)
        || produce_single_len_token(chr).is_some()
        || is_whitespace(chr))
}

#[derive(Clone, Debug)]
pub struct MathTokenizer<'c> {
    pos: usize,
    curr: SpanWrapper<Token>,
    chars: Chars<'c>,
}

impl<'c> MathTokenizer<'c> {
    fn single_span(&self) -> Span {
        Span::new(self.pos, self.pos + 1)
    }

    fn cut_span(&mut self, span: Span) {
        while self.pos < span.end() {
            self.chars.next();
            self.pos += 1;
        }
    }

    fn peek_char(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn consume_while(
        &mut self,
        mut start_predicate: impl FnMut(char) -> bool,
        mut tail_predicate: impl FnMut(char) -> bool,
    ) -> Option<SpanWrapper<String>> {
        if !start_predicate(self.peek_char()) {
            return None;
        }

        let start = self.peek_char().to_string();
        let start_span = self.single_span();

        self.cut_span(start_span.clone());

        let tail = self
            .chars
            .clone()
            .take_while(|chr| tail_predicate(chr.clone()))
            .collect::<String>();
        let tail_span = Span::new(self.pos, self.pos + tail.len());

        self.cut_span(tail_span.clone());

        let val = [start, tail].concat();
        let span = [start_span, tail_span].concat_span();

        Some(SpanWrapper::new(val, span))
    }

    fn produce_next_token(&mut self) -> Option<SpanWrapper<Token>> {
        if self.is_eof() {
            return Some(SpanWrapper::new(Token::Eof, self.single_span()));
        }

        if let Some(tok) = produce_single_len_token(self.peek_char()) {
            let t = Some(SpanWrapper::new(tok, self.single_span()));
            self.cut_span(self.single_span());
            return t;
        }

        if let Some(tok) = self
            .consume_while(is_ident_start, is_ident_tail)
            .map(|w| w.map(|v| Token::Ident(v)))
        {
            return Some(tok);
        }

        if let Some(tok) = self
            .consume_while(is_number_start, is_number_tail)
            .map(|w| w.map(|v| Token::Num(v)))
        {
            return Some(tok);
        }

        self.consume_while(is_unknown, is_unknown)
            .map(|w| w.map(|_| Token::Unknown))
    }
}

impl<'c> Tokenizer<Token> for MathTokenizer<'c> {
    fn curr(&self) -> ir::span::SpanWrapper<Token> {
        self.curr.clone()
    }

    fn first(&self) -> ir::span::SpanWrapper<Token> {
        let mut cloned = self.clone();
        cloned.bump();
        cloned.curr()
    }

    fn second(&self) -> ir::span::SpanWrapper<Token> {
        let mut cloned = self.clone();
        cloned.bump();
        cloned.bump();
        cloned.curr()
    }

    fn is_eof(&self) -> bool {
        self.chars.clone().collect::<Vec<_>>().is_empty() 
    }

    fn bump(&mut self) {
        self.curr = self
            .produce_next_token()
            .unwrap_or(SpanWrapper::new(Token::Eof, self.single_span()));
    }
}
*/
