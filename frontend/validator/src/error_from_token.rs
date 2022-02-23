use error::Error;
use token::Token;

pub trait FromToken {
    fn from_token(input_expr: &String, msg: &str, token: &Token) -> Error {
        let span = token.span();
        let start = span.start();
        let end = span.end();

        Error::new(
            input_expr,
            msg.to_string(),
            start,
            end,
        )
    }
}

impl FromToken for Error {}
