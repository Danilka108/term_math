use std::cmp::Ordering;
use crate::span::SpanWrapper;

const NEG_OP_PRIORITY: usize = 1;

const ADD_OP_PRIORITY: usize = 1;
const SUB_OP_PRIORITY: usize = 1;
const DIV_OP_PRIORITY: usize = 2;
const MUL_OP_PRIORITY: usize = 2;

#[derive(Clone, Debug)]
pub enum UnOpKind {
    Neg,
}

impl UnOpKind {
    fn priority(&self) -> usize {
        match self {
            Self::Neg => NEG_OP_PRIORITY,
        }
    }
}

impl PartialOrd for UnOpKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for UnOpKind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl PartialEq for UnOpKind {
    fn eq(&self, rhs: &Self) -> bool {
        matches!(self.cmp(rhs), Ordering::Equal)
    }
}

impl Eq for UnOpKind {}

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

#[derive(Clone, Debug)]
pub enum Node {
    Num(String),
    BinOp(BinOpKind, Box<SpanWrapper<Node>>, Box<SpanWrapper<Node>>),
    UnOp(UnOpKind, Box<SpanWrapper<Node>>),
    Fn(String, Vec<SpanWrapper<Node>>),
}
