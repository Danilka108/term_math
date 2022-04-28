/*
        match self.second().to_tuple() {
            (Token::Lit(LitKind::Comma) | Token::CloseDelim(DelimKind::Paren), span) => {
                return Err(SpanWrapper::new(
                    ERR__EMPTY_ARG.to_owned(),
                    [separator_span.clone(), span].concat_span(),
                ))
            }
            _ => (),
        }
*/

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

use ir::ast::Node;
use ir::span::SpanWrapper;
use lexer::Lexer;
use parser::Parser;

pub fn parse(expr: String) -> Result<Box<SpanWrapper<Node>>, SpanWrapper<String>> {
    let token_stream = Lexer::new(expr.as_str()).tokenize();
    Parser::new(token_stream).build_ast()
}
