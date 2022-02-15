use crate::parser::Parser;
use ast::{AstNode, NumberNode};
use token::TokenKind;
use error::FrontendError;

impl Parser {
    pub(crate) fn parse_number(&mut self) -> Result<(), FrontendError> {
        let number_val = match self.get_curr_token_kind() {
            Some(TokenKind::Number(val)) => val,
            _ => return Ok(()),
        };

        self.output
            .push(AstNode::Number(NumberNode::new(number_val)));

        Ok(())
    }
}
