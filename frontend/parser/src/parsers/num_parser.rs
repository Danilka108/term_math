use crate::parser::Parser;
use ast::node::{AstNode, NumNode, NodeKind};
use token::TokenKind;
use error::Error;

impl Parser {
    pub(crate) fn parse_number(&mut self) -> Result<(), Error> {
        let number_val = match self.get_curr_token_kind() {
            Some(TokenKind::Number(val)) => val,
            _ => return Ok(()),
        };

        let span = match self.token_stream.curr() {
            Some(token) => token.span(),
            _ => return Ok(()),
        };

        self.output
            .push(AstNode::new(NodeKind::Num(NumNode::new(number_val)), span));

        Ok(())
    }
}
