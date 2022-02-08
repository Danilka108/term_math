use frontend::Frontend;

fn main() {
    match Frontend::new("(8 + 2 * 5) / (1 + 3 * 2 - 4)").build_ast() {
        Ok(ast) => {
            dbg!(ast);
            ()
        },
        Err(error) => {
            print!("{}", error);
        }, 
    };
}
