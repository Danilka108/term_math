use notification::Notification;
use token::Token;

pub trait ErrorFromToken {
    fn new_error_from_token(input_expr: &String, msg: &str, token: &Token) -> Notification {
        let span = token.span();
        let start = span.start();
        let end = span.end();

        Notification::new_error(
            input_expr,
            msg.to_string(),
            start,
            end,
        )
    }
}

impl ErrorFromToken for Notification {}
