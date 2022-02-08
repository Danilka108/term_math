pub use ast::node::AstNode;
pub use error::FrontendError;
use lexer::Lexer;
use parser::Parser;
use validator::Validator;

pub struct Frontend {
    expr: String,
}

impl Frontend {
    pub fn new(expr: &str) -> Self {
        Self { expr: expr.to_string() }
    }

    pub fn build_ast(self) -> Result<AstNode, FrontendError> {
        let token_stream = &mut Lexer::new(&self.expr).lex();
        Validator::new(token_stream).validate()?;
        Parser::new(token_stream).parse()
    }
}
