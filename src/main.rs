use ast::node::AstNode;
use notification::Notification;
use frontend::Frontend;
use backend::Backend;

fn build_ast_from_user_input() -> Result<Box<AstNode>, Notification> {
    Ok(Frontend::from_user_input()?.build_ast()?)
}

fn build_ast_from_str(expr: &str) -> Result<Box<AstNode>, Notification> {
    Ok(Frontend::from_str(expr).build_ast()?)
}

fn main() {
    // + -
    // (1) (324 * 3) (-5)
    
    // - +
    // (-5) (324 * 3) (1)
    let expr = "1 / 0";

    let ast = match build_ast_from_str(expr) {
        Ok(ast) => ast,
        Err(error) => {
            print!("{}", error);
            return;
        },
    };

    dbg!(ast.clone());

    let (result, notifications) = Backend::new(expr, ast).traverse_ast();

    for notification in notifications {
        print!("{}", notification);
    }

    println!("Expression result: {}", result);
}
