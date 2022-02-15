#[derive(Debug, Clone)]
pub enum LiteralToken {
    Asterisk,
    Slash,
    Plus,
    Hyphen,
    Comma,
}

#[derive(Debug, Clone)]
pub enum DelimToken {
    Paren,
    Brace,
    Bracket,
}

impl DelimToken {
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
    Literal(LiteralToken),

    OpenDelim(DelimToken),
    CloseDelim(DelimToken),

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
pub struct TokenSpan {
    start: usize,
    end: usize,
}

impl TokenSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn concat(&self, other: &Self) -> Self {
        Self::new(self.start, other.end)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    span: TokenSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TokenSpan) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }

    pub fn span(&self) -> TokenSpan {
        self.span.clone()
    }
}
