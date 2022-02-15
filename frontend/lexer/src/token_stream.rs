use token::Token;

#[derive(Debug, Clone)]
pub struct TokenStream {
    expr: String,
    tokens: Vec<Token>,
    caret_pos: Option<usize>,
}

impl TokenStream {
    pub fn new(expr: String, tokens: Vec<Token>) -> Self {
        Self { expr, tokens, caret_pos: None }
    }

    pub fn expr(&self) -> &String {
        &self.expr
    }

    pub fn to_next(&mut self) -> Option<&Token> {
        self.caret_pos = match self.caret_pos {
            Some(pos) if (pos as i64) < (self.tokens.len() as i64) - 1 => Some(pos + 1),
            None => Some(0),
            _ => None,
        };

        self.tokens.get(self.caret_pos?)
    }

    pub fn to_prev(&mut self) -> Option<&Token> {
        self.caret_pos = match self.caret_pos {
            Some(pos) if pos > 0 => Some(pos - 1),
            _ => None,
        };

        self.tokens.get(self.caret_pos?)
    }

    pub fn curr(&self) -> Option<&Token> {
        self.tokens.get(self.caret_pos?)
    }

    pub fn next(&self) -> Option<&Token> {
        self.tokens.get(self.caret_pos? + 1)
    }

    pub fn prev(&self) -> Option<&Token> {
        let prev_pos = (self.caret_pos? as i64) - 1;
        self.tokens.get(if prev_pos < 0 { return None } else { prev_pos as usize })
    }
}
