use frontend::{Frontend, FrontendError};
use ast::AstNode;

fn build_ast_from_user_input() -> Result<AstNode, FrontendError> {
    Ok(Frontend::from_user_input()?.build_ast()?)
}

fn build_ast_from_str(expr: &str) -> Result<AstNode, FrontendError> {
    Ok(Frontend::from_str(expr).build_ast()?)
}

fn main() {
    match build_ast_from_user_input() {
        Ok(ast) => {
            dbg!(ast);
            ()
        },
        Err(error) => {
            print!("{}", error);
        }, 
    };
}
