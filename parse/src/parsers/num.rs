use super::errors::*;
use super::parser::*;
use ir::ast::*;
use ir::span::*;
use ir::token::*;

impl Parser {
    fn is_valid_num_bounds(&self) -> bool {
        let is_valid_lhs = matches!(
            self.curr().val(),
            Token::Lit(
                LitKind::Asterisk
                    | LitKind::Slash
                    | LitKind::Plus
                    | LitKind::Hyphen
                    | LitKind::Comma
            ) | Token::OpenDelim(_)
                | Token::Eof
        );
        let is_valid_rhs = matches!(
            self.second().val(),
            Token::Lit(
                LitKind::Asterisk
                    | LitKind::Slash
                    | LitKind::Plus
                    | LitKind::Hyphen
                    | LitKind::Comma
            ) | Token::CloseDelim(_)
                | Token::Eof
        );

        let is_lhs_separator = matches!(self.curr().val(), Token::Eof);
        let is_rhs_separator = matches!(self.second().val(), Token::Eof);

        if is_lhs_separator && is_rhs_separator {
            return false;
        }

        is_valid_rhs && is_valid_lhs
    }

    pub(crate) fn parse_num(&mut self) -> PResult {
        let (token, span) = self.first().to_tuple();

        let val = match token {
            Token::Num(val) => val,
            _ => return Ok(()),
        };

        if !self.is_valid_num_bounds() {
            return Self::new_err(ERR__MISSING_OPERATOR, span);
        }

        let node = SpanWrapper::new(Node::Num(val), span);
        self.push_node(node);

        Ok(())
    }
}
