use super::errors::{ERR__INVALID_LEFT_OPERAND, ERR__INVALID_RIGHT_OPERAND};
use super::*;
use ir::ast::*;
use ir::cursor::prelude::*;
use ir::cursor::TokenCursor;
use ir::token::*;
use util::*;

pub fn is_bin_op<'tok>(token: &'tok Token) -> bool {
    matches!(
        token,
        Token::Lit(LitKind::Plus | LitKind::Asterisk | LitKind::Hyphen | LitKind::Slash)
    )
}

fn is_un_op<'tok>(token: &'tok Token) -> bool {
    matches!(token, Token::Lit(LitKind::Hyphen))
}

pub fn is_op<'tok>(token: &'tok Token) -> bool {
    is_un_op(token) || is_bin_op(token)
}

fn is_right_operand<'tok>(token: &'tok Token) -> bool {
    matches!(token, Token::Num(_) | Token::OpenDelim(_) | Token::Ident(_))
}

fn is_left_operand<'tok>(token: &'tok Token) -> bool {
    matches!(token, Token::Num(_) | Token::CloseDelim(_))
}

fn produce_bin_op<'stream>(
    mut token_stream: SharedTokenStream<'stream>,
) -> ProduceRes<BuffElement> {
    let kind = match token_stream.next().unwrap().val() {
        Token::Lit(LitKind::Plus) => BinOpKind::Add,
        Token::Lit(LitKind::Hyphen) => BinOpKind::Sub,
        Token::Lit(LitKind::Asterisk) => BinOpKind::Mul,
        Token::Lit(LitKind::Slash) => BinOpKind::Div,
        _ => return ProduceRes::None,
    };

    ProduceRes::Ok(BuffElement::BinOp(kind))
}

fn produce_un_op<'stream>(mut token_stream: SharedTokenStream<'stream>) -> ProduceRes<BuffElement> {
    match token_stream.next().unwrap().val() {
        Token::Lit(LitKind::Hyphen) => ProduceRes::Ok(BuffElement::UnOp(UnOpKind::Neg)),
        _ => ProduceRes::None,
    }
}

fn consume_bin_op<'cur>(cursor: TokenCursor<'cur>) -> ConsumeRes<BuffElement> {
    cursor
        .consume_once()
        .look_at_first()
        .expect(is_bin_op)
        .look_at_second()
        .require(is_right_operand, ERR__INVALID_RIGHT_OPERAND.to_owned())
        .look_at_curr()
        .require(is_left_operand, ERR__INVALID_LEFT_OPERAND.to_owned())
        .produce(produce_bin_op)
}

fn consume_un_op<'cur>(cursor: TokenCursor<'cur>) -> ConsumeRes<BuffElement> {
    cursor
        .consume_once()
        .look_at_first()
        .expect(is_un_op)
        .look_at_second()
        .require(is_right_operand, ERR__INVALID_RIGHT_OPERAND.to_owned())
        .produce(produce_un_op)
}

pub trait BuildOp<NEW> {
    fn build_op(&mut self, new: &NEW) -> Node;
}

pub trait BuildLastOp<NEW> {
    fn build_last_op(&mut self, new: &NEW);
}

impl<'expr> BuildOp<BinOpKind> for Expr<'expr> {
    fn build_op(&mut self, kind: &BinOpKind) -> Node {
        let right_operand = match self.nodes.pop() {
            Some(node) => node,
            _ => panic!("Right operand must exist when building the binary operator"),
        };
        let left_operand = match self.nodes.pop() {
            Some(node) => node,
            _ => panic!("Left operand must exist when a building the binary operator"),
        };

        Node::BinOp(
            kind.clone(),
            Box::new(left_operand),
            Box::new(right_operand),
        )
    }
}

impl<'expr> BuildOp<UnOpKind> for Expr<'expr> {
    fn build_op(&mut self, kind: &UnOpKind) -> Node {
        let operand = match self.nodes.pop() {
            Some(node) => node,
            _ => panic!("Operand must exist when a building the unary operator"),
        };

        Node::UnOp(kind.clone(), Box::new(operand))
    }
}

impl<'expr> BuildLastOp<BinOpKind> for Expr<'expr> {
    fn build_last_op(&mut self, new_op_kind: &BinOpKind) {
        let last_buff_elem = match self.buffer.pop() {
            Some(elem) => elem,
            None => return,
        };

        let builded_op = match last_buff_elem.borrow_val() {
            BuffElement::UnOp(last_op_kind) => self.build_op(last_op_kind),
            BuffElement::BinOp(last_op_kind) if last_op_kind >= new_op_kind => {
                self.build_op(last_op_kind)
            }
            _ => {
                self.buffer.push(last_buff_elem);
                return;
            }
        };

        let node = SpanWrapper::new(builded_op, last_buff_elem.span().clone());
        self.nodes.push(node);
    }
}

impl<'expr> BuildLastOp<UnOpKind> for Expr<'expr> {
    fn build_last_op(&mut self, new_op_kind: &UnOpKind) {
        let last_buff_elem = match self.buffer.pop() {
            Some(elem) => elem,
            None => return,
        };

        let builded_op = match last_buff_elem.borrow_val() {
            BuffElement::UnOp(last_op_kind) if last_op_kind >= new_op_kind => {
                self.build_op(last_op_kind)
            }
            _ => {
                self.buffer.push(last_buff_elem);
                return;
            }
        };

        let node = SpanWrapper::new(builded_op, last_buff_elem.span().clone());
        self.nodes.push(node);
    }
}

impl<'expr> BuildLastOp<SpanWrapper<BuffElement>> for Expr<'expr> {
    fn build_last_op(&mut self, new_buff_elem: &SpanWrapper<BuffElement>) {
        match new_buff_elem.borrow_val() {
            BuffElement::BinOp(kind) => self.build_last_op(kind),
            BuffElement::UnOp(kind) => self.build_last_op(kind),
            _ => (),
        }
    }
}

impl<'expr> Expr<'expr> {
    pub fn parse_op(&mut self) -> ParseRes<'expr> {
        let buff_elem =
            modify!(consume_bin_op(self.cursor.clone()).or(consume_un_op(self.cursor.clone())));

        self.cursor.cut_by_span(buff_elem.borrow_span().clone());
        self.build_last_op(&buff_elem);

        self.buffer.push(buff_elem);

        ParseRes::None
    }
}
