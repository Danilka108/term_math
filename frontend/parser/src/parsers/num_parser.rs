use crate::parser::Parser;
use ast::node::{AstNode, NumNode};
use notification::Notification;
use token::TokenKind;

impl Parser {
    pub(crate) fn parse_number(&mut self) -> Result<(), Notification> {
        let number_val = match self.get_curr_token_kind() {
            Some(TokenKind::Number(val)) => val,
            _ => return Ok(()),
        };

        let span = match self.token_stream.curr() {
            Some(token) => token.span(),
            _ => return Ok(()),
        };

        let node = AstNode::Num(NumNode::new(number_val, span));
        self.output.push(Box::new(node));

        Ok(())
    }
}
