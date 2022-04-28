use super::errors::*;
use super::parser::*;
use ir::ast::*;
use ir::span::*;
use ir::token::*;

impl Parser {
    fn is_empty_curr(&self) -> bool {
        matches!(self.curr().val(), Token::Eof)
    }

    fn is_empty_first(&self) -> bool {
        matches!(self.first().val(), Token::Eof)
    }

    fn is_empty_second(&self) -> bool {
        matches!(self.second().val(), Token::Eof)
    }

    fn is_valid_right_operand(&self) -> bool {
        matches!(
            self.second().val(),
            Token::OpenDelim(_) | Token::Num(_) | Token::Ident(_)
        )
    }

    fn is_valid_left_operand(&self) -> bool {
        matches!(self.curr().val(), Token::CloseDelim(_) | Token::Num(_))
    }

    fn build_bin_op(&mut self, kind: BinOpKind, span: Span) -> PResult {
        let rhs = self.pop_node_or(ERR__MISSING_RIGHT_OPERAND, span.clone())?;
        let lhs = self.pop_node_or(ERR__INVALID_LEFT_OPERAND, span.clone())?;

        let node = SpanWrapper::new(Node::BinOp(kind, Box::new(lhs), Box::new(rhs)), span);

        self.push_node(node);
        Ok(())
    }

    fn build_un_op(&mut self, kind: UnOpKind, span: Span) -> PResult {
        let operand = self.pop_node_or(ERR__INVALID_RIGHT_OPERAND, span.clone())?;

        let node = SpanWrapper::new(Node::UnOp(kind, Box::new(operand)), span);
        self.push_node(node);

        Ok(())
    }

    pub(crate) fn collect_ops(&mut self) -> PResult {
        loop {
            let (val, span) = match self.pop_buff_tuple() {
                Some(t) => t,
                None => break,
            };

            match val {
                BuffElem::BinOp(kind) => self.build_bin_op(kind, span)?,
                BuffElem::UnOp(kind) => self.build_un_op(kind, span)?,
                _ => {
                    self.push_buff_tuple(val, span);
                    break;
                }
            }
        }

        Ok(())
    }

    fn parse_bin_op(&mut self) -> PResult<bool> {
        let (new_op_val, new_op_span) = self.first().to_tuple();

        let new_op_kind = match new_op_val {
            Token::Lit(LitKind::Plus) => BinOpKind::Add,
            Token::Lit(LitKind::Hyphen) => BinOpKind::Sub,
            Token::Lit(LitKind::Asterisk) => BinOpKind::Mul,
            Token::Lit(LitKind::Slash) => BinOpKind::Div,
            _ => return Ok(false),
        };

        if self.is_empty_second() {
            return Self::new_err(ERR__MISSING_RIGHT_OPERAND, new_op_span);
        }

        if !self.is_valid_right_operand() {
            return Self::new_err(ERR__INVALID_RIGHT_OPERAND, new_op_span);
        }

        if self.is_empty_curr() {
            return Self::new_err(ERR__MISSING_LEFT_OPERAND, new_op_span);
        }

        if !self.is_valid_left_operand() {
            return Self::new_err(ERR__INVALID_LEFT_OPERAND, new_op_span);
        }

        let (last_op_val, last_op_span) = match self.pop_buff_tuple() {
            Some(t) => t,
            None => {
                self.push_buff_tuple(BuffElem::BinOp(new_op_kind), new_op_span);
                return Ok(true);
            }
        };

        match last_op_val {
            BuffElem::BinOp(last_op_kind) if last_op_kind >= new_op_kind => {
                self.build_bin_op(last_op_kind, last_op_span)?
            }
            BuffElem::UnOp(last_op_kind) => self.build_un_op(last_op_kind, last_op_span)?,
            _ => self.push_buff_tuple(last_op_val, last_op_span),
        }

        self.push_buff_tuple(BuffElem::BinOp(new_op_kind), new_op_span);
        Ok(true)
    }

    fn parse_un_op(&mut self) -> PResult<bool> {
        let (op_val, op_span) = self.first().to_tuple();

        let op_kind = match op_val {
            Token::Lit(LitKind::Hyphen) => UnOpKind::Neg,
            _ => return Ok(false),
        };

        if self.is_empty_first() {
            return Self::new_err(ERR__MISSING_RIGHT_OPERAND, op_span);
        }

        if !self.is_valid_right_operand() {
            return Self::new_err(ERR__INVALID_RIGHT_OPERAND, op_span);
        }

        self.push_buff_tuple(BuffElem::UnOp(op_kind), op_span);
        Ok(true)
    }

    pub(crate) fn parse_op(&mut self) -> PResult {
        match self.parse_bin_op() {
            Ok(true) => return Ok(()),
            Ok(false) => match self.parse_un_op() {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            },
            Err(err) => match self.parse_un_op() {
                Ok(true) => Ok(()),
                Ok(false) => Err(err),
                Err(err_2) => Err(err_2),
            },
        }
    }
}
