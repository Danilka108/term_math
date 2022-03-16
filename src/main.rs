use ast::node::AstNode;
use backend::Backend;
use frontend::Frontend;
use notification::Notification;
use number_2::FloatNumber;

fn build_ast_from_user_input() -> Result<Box<AstNode>, Notification> {
    Ok(Frontend::from_user_input()?.build_ast()?)
}

fn build_ast_from_str(expr: &str) -> Result<Box<AstNode>, Notification> {
    Ok(Frontend::from_str(expr).build_ast()?)
}

fn main() {
    let a = FloatNumber::from_unsigned_numeric_string("1111".to_string()).unwrap();
    let b = FloatNumber::from_unsigned_numeric_string("9999".to_string()).unwrap();
    let c = a.mul(b);

    dbg!(c);
    /*
    let expr = "1 / 0";

    let ast = match build_ast_from_str(expr) {
        Ok(ast) => ast,
        Err(error) => {
            print!("{}", error);
            return;
        }
    };

    dbg!(ast.clone());

    let (result, notifications) = Backend::new(expr, ast).traverse_ast();

    for notification in notifications {
        print!("{}", notification);
    }

    println!("Expression result: {}", result);
    */
}
