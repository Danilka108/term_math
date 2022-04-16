use std::cmp::Ordering;

use crate::span::SpanWrapper;
use number::Dec64;

const ADD_OP_PRIORITY: usize = 2;
const SUB_OP_PRIORITY: usize = 2;
const DIV_OP_PRIORITY: usize = 1;
const MUL_OP_PRIORITY: usize = 1;

pub type NumNode = SpanWrapper<Dec64>;

#[derive(Clone, Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOpKind {
    fn priority(&self) -> usize {
        match self {
            Self::Add => ADD_OP_PRIORITY,
            Self::Sub => SUB_OP_PRIORITY,
            Self::Mul => MUL_OP_PRIORITY,
            Self::Div => DIV_OP_PRIORITY,
        }
    }
}

impl PartialOrd for BinOpKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for BinOpKind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl PartialEq for BinOpKind {
    fn eq(&self, rhs: &Self) -> bool {
        matches!(self.cmp(rhs), Ordering::Equal)
    }
}

impl Eq for BinOpKind {}

pub trait OpOperands {
    type Operands;

    fn operands(self) -> Self::Operands;
}

pub trait OpKind {
    type Kind;

    fn kind(&self) -> &Self::Kind;
}

pub type BinOpNode = SpanWrapper<(Box<AstNode>, Box<AstNode>, BinOpKind)>;

impl OpOperands for BinOpNode {
    type Operands = (Box<AstNode>, Box<AstNode>);

    fn operands(self) -> Self::Operands {
        let val = self.val();
        (val.0, val.1)
    }
}

impl OpKind for BinOpNode {
    type Kind = BinOpKind;

    fn kind(&self) -> &Self::Kind {
        &self.borrow_val().2
    }
}

#[derive(Clone, Debug)]
pub enum UnOpKind {
    Neg,
}

pub type UnOpNode = SpanWrapper<(Box<AstNode>, UnOpKind)>;

impl OpOperands for UnOpNode {
    type Operands = Box<AstNode>;

    fn operands(self) -> Self::Operands {
        self.val().0
    }
}

impl OpKind for UnOpNode {
    type Kind = UnOpKind;

    fn kind(&self) -> &Self::Kind {
        &self.borrow_val().1
    }
}

pub type FnNode = SpanWrapper<(String, Vec<AstNode>)>;

trait FnNodeArgs {
    fn args(self) -> Vec<AstNode>;
}

trait FnNodeName {
    fn name(&self) -> String;
}

impl FnNodeArgs for FnNode {
    fn args(self) -> Vec<AstNode> {
        self.val().1
    }
}

impl FnNodeName for FnNode {
    fn name(&self) -> String {
        self.borrow_val().0.clone()
    }
}

#[derive(Clone, Debug)]
pub enum AstNode {
    Num(NumNode),
    BinOp(BinOpNode),
    UnOp(UnOpNode),
    Fn(FnNode),
}
