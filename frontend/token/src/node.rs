use ast::node::{OperatorNode, OperatorKind};
use crate::token::{TokenKind, LiteralToken, Token};

pub trait FromToken {
    fn from_token(token: &Token) -> Option<OperatorNode> {
        use OperatorKind::*;

        Some(match token.kind() {
            TokenKind::Literal(literal) => match literal {
                LiteralToken::Plus => OperatorNode::new(Addition),
                LiteralToken::Hyphen => OperatorNode::new(Subtraction),
                LiteralToken::Asterisk => OperatorNode::new(Multiplication),
                LiteralToken::Slash => OperatorNode::new(Division),
                _ => return None,
            },
            _ => return None,
        })
    }

}

impl FromToken for OperatorNode {}
