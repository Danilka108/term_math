use ast::node::{AstNode, NodeKind, NumberNode};
use ast::span::Span;
use backend::Number;
use frontend::{Frontend, FrontendError};

fn build_ast_from_user_input() -> Result<AstNode, FrontendError> {
    Ok(Frontend::from_user_input()?.build_ast()?)
}

fn build_ast_from_str(expr: &str) -> Result<AstNode, FrontendError> {
    Ok(Frontend::from_str(expr).build_ast()?)
}

fn main() {
    // let number_value_1 = "88.00324234";
    let number_value_1 = "123456789123456789.123456789";
    let number_value_2 = "987654321.987654321";

    let number_node_1 = NumberNode::new(String::from(number_value_1));
    let number_node_2 = NumberNode::new(String::from(number_value_2));

    let num_1 = Number::from_number_node(number_node_1);
    let num_2 = Number::from_number_node(number_node_2);

    dbg!(num_1 / num_2);
    /*
    match build_ast_from_user_input() {
        Ok(ast) => {
            dbg!(ast);
            ()
        },
        Err(error) => {
            print!("{}", error);
        },
    };
    */
}
