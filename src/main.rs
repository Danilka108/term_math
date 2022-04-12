use ast::node::AstNode;
use backend::Backend;
use frontend::Frontend;
use notification::Notification;
use number_2::{Dec64, TryFromStrError};

fn build_ast_from_user_input() -> Result<Box<AstNode>, Notification> {
    Ok(Frontend::from_user_input()?.build_ast()?)
}

fn build_ast_from_str(expr: &str) -> Result<Box<AstNode>, Notification> {
    Ok(Frontend::from_str(expr).build_ast()?)
}

fn main() -> Result<(), TryFromStrError> {

    let a = Dec64::try_from("1111")?;
    let b = Dec64::try_from("3")?;

    dbg!((a / b).to_string());
    dbg!((1111_f64 / 3_f64).to_string());

    Ok(())
}
