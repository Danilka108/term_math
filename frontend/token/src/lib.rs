use ast::span::Span;

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Asterisk,
    Slash,
    Plus,
    Hyphen,
    Comma,
}

#[derive(Debug, Clone)]
pub enum DelimKind {
    Paren,
    Brace,
    Bracket,
}

impl DelimKind {
    pub fn is_eq(&self, other: &Self) -> bool {
        match (self.clone(), other.clone()) {
            (Self::Paren, Self::Paren)
            | (Self::Brace, Self::Brace)
            | (Self::Bracket, Self::Bracket) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Literal(LiteralKind),

    OpenDelim(DelimKind),
    CloseDelim(DelimKind),

    Number(String),
    Ident(String),

    Error(String),

    Eof,
}

impl TokenKind {
    pub fn is_delim(&self) -> bool {
        match self {
            Self::OpenDelim(_) | Self::CloseDelim(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Self::Number(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }
}
