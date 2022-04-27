use super::errors::ERR__EMPTY_EXPR;
use super::expr::{Expr, ExprKind};
use ir::ast::*;
use ir::cursor::prelude::*;
use ir::cursor::TokenCursor;
use ir::span::*;
use ir::token::*;

fn is_whitespace(token: &SpanWrapper<&Token>) -> bool {
    matches!(token.borrow_val(), Token::Whitespace)
}

fn is_eof(token: &SpanWrapper<&Token>) -> bool {
    matches!(token.borrow_val(), Token::Eof)
}

fn get_token_cursor<'stream>(
    token_stream: &'stream TokenStream,
) -> Result<TokenCursor<'stream>, SpanWrapper<String>> {
    let filtered = token_stream
        .to_shared_stream()
        .filter(|t| !is_whitespace(t) && !is_eof(t));

    if filtered.clone().count() == 0 {
        return Err(SpanWrapper::new(
            ERR__EMPTY_EXPR.to_owned(),
            Span::new(0, 0),
        ));
    }

    Ok(filtered
        .clone()
        .collect::<Vec<_>>()
        .into_iter()
        .into_cursor())
}

pub fn build_ast(token_stream: TokenStream) -> Result<Box<SpanWrapper<Node>>, SpanWrapper<String>> {
    let mut func_args = Vec::new();
    let mut delimited = None;

    let mut exprs = vec![Expr::new(
        ExprKind::Global,
        get_token_cursor(&token_stream)?,
    )];

    let ast = loop {
        let mut expr = if let Some(expr) = exprs.pop() {
            expr
        } else {
            panic!("Parser loop can only be stopped at the frame matching");
        };

        expr.attach_fn_args(&mut func_args);
        expr.attach_delimited(delimited);
        delimited = None;

        match expr.parse() {
            Modification::Ok(additional_exprs) => {
                exprs.push(expr);
                exprs = [exprs, additional_exprs].concat();
                continue;
            }
            Modification::Err(err) => return Err(err),
            Modification::None => (),
        }

        match expr.kind() {
            ExprKind::FnArg => {
                func_args.push(expr.collect_nodes());
                continue;
            }
            ExprKind::Delimited => {
                delimited = Some(expr.collect_nodes());
                continue;
            }
            ExprKind::Global => {
                break Box::new(expr.collect_nodes());
            }
        }
    };

    Ok(ast)
}
