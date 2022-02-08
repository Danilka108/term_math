use crate::combiner::Combiner;
use crate::constants::ERR__UNKNOWN_SYMBOLS;
use crate::symbol_stream::SymbolStream;
pub use crate::token_stream::TokenStream;
use ast::token::{Token, TokenKind, TokenSpan};

pub struct Lexer<'s> {
    pub(crate) input_expr: String,
    pub(crate) symbol_stream: SymbolStream<'s>,
    combiner: Combiner,
    tokens: Vec<Token>,
}

impl<'s> Lexer<'s> {
    const LEXERS: [fn(&mut Self) -> Option<Token>; 5] = [
        Self::lex_whitespace,
        Self::lex_literal,
        Self::lex_delim,
        Self::lex_number,
        Self::lex_ident,
    ];

    pub fn new(expr: &'s String) -> Self {
        Self {
            input_expr: expr.clone(),
            symbol_stream: SymbolStream::new(expr),
            combiner: Combiner::new(),
            tokens: Vec::new(),
        }
    }

    pub(crate) fn lex_char(
        &mut self,
        identify_kind: impl FnOnce(char) -> Option<TokenKind>,
    ) -> Option<Token> {
        let next = self.symbol_stream.next()?;
        let kind = identify_kind(next)?;

        self.symbol_stream.to_next();

        let pos = self.symbol_stream.pos()?;
        let span = TokenSpan::new(pos, pos + 1);

        Some(Token::new(kind, span))
    }

    pub(crate) fn lex_while(
        &mut self,
        mut predicate: impl FnMut(char) -> bool,
    ) -> Option<(TokenSpan, String)> {
        let start_pos = if let Some(pos) = self.symbol_stream.pos() {
            pos + 1
        } else {
            0
        };
        let mut end_pos = start_pos;
        let mut val = String::new();

        while let Some(sym) = self.symbol_stream.next() {
            if !predicate(sym) {
                break;
            }

            self.symbol_stream.to_next();
            val.push(sym);
            end_pos += 1;
        }

        if val.len() != 0 {
            let span = TokenSpan::new(start_pos, end_pos);
            Some((span, val))
        } else {
            None
        }
    }

    fn push_token(&mut self, token: Option<Token>) {
        if let Some(token) = token {
            if let Some((span, _)) = self.combiner.combine() {
                let error_token =
                    Token::new(TokenKind::Error(ERR__UNKNOWN_SYMBOLS.to_string()), span);
                self.tokens.push(error_token);
            }

            self.tokens.push(token);
        }
    }

    pub fn lex(mut self) -> TokenStream {
        'l: loop {
            for lex in Self::LEXERS {
                let token = lex(&mut self);

                if token.is_none() {
                    continue;
                }

                self.push_token(token);

                continue 'l;
            }

            if self.symbol_stream.is_eof() {
                let token = self.lex_eof();
                self.push_token(token);

                break;
            }

            match (self.symbol_stream.to_next(), self.symbol_stream.pos()) {
                (Some(sym), Some(pos)) => self.combiner.push(sym, pos),
                _ => (),
            }
        }

        TokenStream::new(self.input_expr, self.tokens)
    }
}
