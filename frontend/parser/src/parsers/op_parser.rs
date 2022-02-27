use crate::parser::{BufferElement, Parser};
use ast::node::{AstNode, BinOpKind, BinOpNode, UnOpKind, UnOpNode};
use ast::span::Span;
use notification::Notification;
use token::{LiteralKind, TokenKind};

impl Parser {
    fn get_pair_operands(&mut self) -> Result<(Box<AstNode>, Box<AstNode>), Notification> {
        let first_operand = if let Some(node) = self.output.pop() {
            node
        } else {
            return Err(self.get_unknown_err());
        };

        let second_operand = if let Some(node) = self.output.pop() {
            node
        } else {
            return Err(self.get_unknown_err());
        };

        Ok((second_operand, first_operand))
    }

    fn get_signle_operand(&mut self) -> Result<Box<AstNode>, Notification> {
        if let Some(node) = self.output.pop() {
            Ok(node)
        } else {
            Err(self.get_unknown_err())
        }
    }

    fn build_bin_op(&mut self, kind: BinOpKind, span: Span) -> Result<(), Notification> {
        let (left_operand, right_operand) = self.get_pair_operands()?;

        let node = AstNode::BinOp(BinOpNode::new(kind, left_operand, right_operand, span));
        self.output.push(Box::new(node));

        Ok(())
    }

    fn build_un_op(&mut self, kind: UnOpKind, span: Span) -> Result<(), Notification> {
        let operand = self.get_signle_operand()?;

        let node = AstNode::UnOp(UnOpNode::new(kind, operand, span));
        self.output.push(Box::new(node));

        Ok(())
    }

    fn reverse_build_bin_op(&mut self, kind: BinOpKind, span: Span) -> Result<(), Notification> {
        let (right_operand, left_operand) = self.get_pair_operands()?;

        let node = AstNode::BinOp(BinOpNode::new(kind, left_operand, right_operand, span));
        self.output.push(Box::new(node));

        Ok(())
    }

    pub(crate) fn parse_op(&mut self) -> Result<(), Notification> {
        let is_unary = match self.get_prev_token_kind() {
            Some(kind) => match kind {
                TokenKind::OpenDelim(_)
                | TokenKind::CloseDelim(_)
                | TokenKind::Literal(LiteralKind::Comma) => true,
                _ => false,
            },
            _ => true,
        };

        let curr_span = match self.token_stream.curr() {
            Some(token) => token.span(),
            _ => return Ok(()),
        };

        match self.get_curr_token_kind() {
            Some(TokenKind::Literal(LiteralKind::Hyphen)) if is_unary => {
                self.buffer
                    .push(BufferElement::UnOp(UnOpKind::Neg, curr_span));
                return Ok(());
            }
            _ => (),
        }

        let new_bin_op_kind = match self.get_curr_token_kind() {
            Some(TokenKind::Literal(lit_kind)) => match lit_kind {
                LiteralKind::Plus => BinOpKind::Add,
                LiteralKind::Slash => BinOpKind::Div,
                LiteralKind::Hyphen => BinOpKind::Sub,
                LiteralKind::Asterisk => BinOpKind::Mul,
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        match self.buffer.pop() {
            Some(BufferElement::UnOp(kind, span)) => self.build_un_op(kind, span)?,
            Some(BufferElement::BinOp(last_bin_op_kind, last_bin_op_span))
                if last_bin_op_kind.priority() <= new_bin_op_kind.priority() =>
            {
                self.build_bin_op(last_bin_op_kind, last_bin_op_span)?
            }
            Some(node) => self.buffer.push(node),
            _ => (),
        }

        self.buffer
            .push(BufferElement::BinOp(new_bin_op_kind, curr_span));

        Ok(())
    }

    pub(crate) fn parse_ops(
        &mut self,
        mut predicate: impl FnMut(&BufferElement) -> bool,
    ) -> Result<(), Notification> {
        let mut buffer_slice = Vec::new();
        let mut output_slice = Vec::new();

        let mut pop_output = || match self.output.pop() {
            Some(node) => output_slice.push(node),
            None => (),
        };

        while let Some(buffer_element) = self.buffer.pop() {
            if predicate(&buffer_element) {
                self.buffer.push(buffer_element);
                break;
            }

            match buffer_element {
                BufferElement::UnOp(_, _) | BufferElement::BinOp(_, _) => pop_output(),
                _ => return Err(self.get_unknown_err()),
            }

            buffer_slice.push(buffer_element);
        }

        if buffer_slice.len() != 0 {
            pop_output();
        }

        let prev_buffer = self.buffer.clone();
        let prev_output = self.output.clone();

        self.buffer = buffer_slice;
        self.output = output_slice;

        while let Some(buffer_element) = self.buffer.pop() {
            match buffer_element {
                BufferElement::UnOp(kind, span) => self.build_un_op(kind, span)?,
                BufferElement::BinOp(kind, span) => self.reverse_build_bin_op(kind, span)?,
                _ => return Err(self.get_unknown_err()),
            }
        }

        self.buffer.reverse();
        self.output.reverse();

        self.buffer = [prev_buffer, self.buffer.clone()].concat();
        self.output = [prev_output, self.output.clone()].concat();

        Ok(())

        /*
        loop {
            match self.buffer.pop() {
                Some(BufferElement::UnOp(kind, span)) => self.build_un_op(kind, span)?,
                Some(BufferElement::BinOp(kind, span)) => self.build_bin_op(kind, span)?,
                Some(buffer_element) if predicate(&buffer_element) => {
                    self.buffer.push(buffer_element);
                    break Ok(());
                }
                None => break Ok(()),
                _ => break Err(self.get_unknown_err()),
            }
        }
        */
    }
}
