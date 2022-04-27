mod errors;
mod lexer;
mod parser;

use ir::ast::Node;
use ir::span::SpanWrapper;
use lexer::Lexer;
use parser::Parser;

pub fn parse(expr: &str) -> Result<Box<SpanWrapper<Node>>, SpanWrapper<String>> {
    let token_stream = Lexer::new(expr).tokenize();
    Parser::new(token_stream).build_ast()
}