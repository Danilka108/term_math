use lexer::Lexer;

fn main() {
    //let a = Lexer::new("12 +4/log2(e, 0.3242)").tokenize();
    let a = Lexer::new("12 +4/1lo@#?_!*234?|~) g2 (e, 0.3F2.43)").tokenize();
    dbg!(a.collect::<Vec<_>>());
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
