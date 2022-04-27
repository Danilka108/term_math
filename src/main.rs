use parse::parse;
use notification::Notification;
use ir::span::SpanWrapper;

fn main() {
    let expr = String::from("((- (2 + (0 + 0))) - 4) + 5 + (1 - 324 + fn(-3223 / 1 * 914 - (111  2 /355 * (3 + 4))))");
    let ast = parse(expr.clone());

    match ast {
        Ok(ast) => {
            dbg!(ast);
        },
        Err(wrapper) => {
            let (msg, span) = wrapper.to_tuple();
            let err = Notification::new_error(&expr, msg, span.start(), span.end());
            print!("{}", err);
        } 
    }
}
