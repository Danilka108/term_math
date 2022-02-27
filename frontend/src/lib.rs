use ast::node::AstNode;
use notification::Notification;
use lexer::Lexer;
use parser::Parser;
use validator::Validator;
use std::io;

const ERR__INPUT_ERROR: &str = "Input error";

pub struct Frontend {
    expr: String,
}

impl Frontend {
    pub fn from_str(expr: &str) -> Self {
        Self { expr: expr.to_string() }
    }

    pub fn from_user_input() -> Result<Self, Notification> {
        println!("Enter math expression:");

        let mut buffer = String::new();

        match io::stdin().read_line(&mut buffer) {
            Err(_) => Err(Notification::new_error(&buffer, ERR__INPUT_ERROR.to_string(), 0, 0)),
            _ => Ok(()),
        }?;

        match buffer.pop() {
            Some('\n') | None => (),
            Some(chr) => buffer.push(chr),
        }

        Ok(Self { expr: buffer })
    }

    pub fn build_ast(self) -> Result<Box<AstNode>, Notification> {
        let token_stream = &mut Lexer::new(&self.expr).lex();
        Validator::new(token_stream).validate()?;
        Parser::new(token_stream).parse()
    }
}
