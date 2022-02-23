use ast::node::{OpNode, BinOpNode, BinOpKind, UnOpKind, UnOpNode};
use crate::token::{TokenKind, LiteralToken, Token};

pub trait FromToken {
    fn from_token(token: &Token, is_unary: bool) -> Option<OpNode> {
        match token.kind() {
            TokenKind::Literal(literal) => match literal {
                LiteralToken::Plus => Some(BinOpNode::new(BinOpKind::Add)),
                LiteralToken::Hyphen if is_unary => Some(UnOpNode::new(UnOpKind::Neg)),
                LiteralToken::Hyphen => Some(BinOpNode::new(BinOpKind::Sub)),
                LiteralToken::Asterisk => Some(BinOpNode::new(BinOpKind::Mul)),
                LiteralToken::Slash => Some(BinOpNode::new(BinOpKind::Div)),
                _ => None,
            },
            _ => None,
        } 
    }

}

impl FromToken for OpNode {}
