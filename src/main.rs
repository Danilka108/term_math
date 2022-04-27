use parse::parse;

fn main() {
    let expr = "((- (2 + (0 + 0))) - 4) + 5 + (1 - 324 + (-3223 / 1 * 914 - (111 / 2 /355 * (3 + 4))))";
    //let expr = "fn(23 * (324 -2) + 3, 2, 352)";
    let ast = parse(expr);
    dbg!(ast);
    //let a = Lexer::new("12 +4/log2(e, 0.3242)").tokenize();
    //let ll_stream = ll_lexer::tokenize("12 +4/1lo@#?_!*234?|~) g2 (e, 0.3F2.43)");
    //let stream = lexer::tokenize("12");
}
/*
use ast::node::AstNode;
use backend::Backend;
use frontend::Frontend;
use notification::Notification;

fn build_ast_from_user_input() -> Result<Box<AstNode>, Notification> {
    Ok(Frontend::from_user_input()?.build_ast()?)
}

fn build_ast_from_str(expr: &str) -> Result<Box<AstNode>, Notification> {
    Ok(Frontend::from_str(expr).build_ast()?)
}

fn main() {
    let expr = "3 + 2 * 4";
    let (num, notifications) = match build_ast_from_str(expr) {
        Ok(ast) => {
            dbg!(ast.clone());
            Backend::new(expr, ast).traverse_ast()
        },
        Err(err) => {
            dbg!(err);
            return;
        }
    };

    dbg!(num.to_string());
    dbg!(notifications);
}
*/
