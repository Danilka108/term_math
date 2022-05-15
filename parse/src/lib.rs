macro_rules! matches_or {
    ($expr:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)? $(,)?, $msg:expr, $span:expr $(,)?) => {
        match $expr {
            $($pattern)|+ $( if $guard )? => return Err(SpanWrapper::new($msg.to_owned(), $span)),
            _ => ()
        }
    };
}

macro_rules! matches_or_else {
    ($expr:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)? $(,)?, $ok:expr, $msg:expr, $span:expr $(,)?) => {
        match $expr {
            $($pattern)|+ $( if $guard )? => $ok,
            _ => return Err(SpanWrapper::new($msg.to_owned(), $span)),
        }
    };
}


mod errors;
mod lexer;
mod parser;
mod parsers;
mod tokenizer;

use ir::ast::Node;
use ir::span::SpanWrapper;
use lexer::Lexer;
use parser::Parser;

pub fn parse(expr: &'static str) -> Result<Box<SpanWrapper<Node>>, SpanWrapper<String>> {
    Ok(Box::new(SpanWrapper::new(Node::Num(String::from("sdf")), ir::span::Span::new(0, 0))))
}

use std::fmt::Debug;
trait Tokenizer<T: Clone + Debug> {
    fn curr(&self) -> SpanWrapper<T>;
    fn first(&self) -> SpanWrapper<T>;
    fn second(&self) -> SpanWrapper<T>;
    fn bump(&mut self);
}

