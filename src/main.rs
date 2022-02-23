use ast::node::{AstNode, NumNode};
use backend::Number;
use error::Error;
use frontend::Frontend;

fn build_ast_from_user_input() -> Result<AstNode, Error> {
    Ok(Frontend::from_user_input()?.build_ast()?)
}

fn build_ast_from_str(expr: &str) -> Result<AstNode, Error> {
    Ok(Frontend::from_str(expr).build_ast()?)
}

fn main() {
    // let number_value_1 = "88.00324234";
    let number_value_1 = "00001";
    let number_value_2 = "5";

    let number_node_1 = NumNode::new(String::from(number_value_1));
    let number_node_2 = NumNode::new(String::from(number_value_2));

    let num_1 = Number::from_number_node(number_node_1);
    let num_2 = Number::from_number_node(number_node_2);

    dbg!(num_1 / num_2);

    /*
    match build_ast_from_str("324 + 1") {
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
