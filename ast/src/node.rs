use crate::constants::{
    ADD_OP_PRIORITY, DIV_OP_PRIORITY, MUL_OP_PRIORITY, SUB_OP_PRIORITY,
};
use crate::span::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumNode {
    val: String,
    span: Span,
}

impl NumNode {
    pub fn new(val: String, span: Span) -> Self {
        Self { val, span }
    }

    pub fn val(&self) -> String {
        self.val.clone()
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOpKind {
    pub fn priority(&self) -> usize {
        match self {
            Self::Add => ADD_OP_PRIORITY,
            Self::Sub => SUB_OP_PRIORITY,
            Self::Mul => MUL_OP_PRIORITY,
            Self::Div => DIV_OP_PRIORITY,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinOpNode {
    left_operand: Box<AstNode>,
    right_operand: Box<AstNode>,
    kind: BinOpKind,
    span: Span,
}

impl BinOpNode {
    pub fn new(
        kind: BinOpKind,
        left_operand: Box<AstNode>,
        right_operand: Box<AstNode>,
        span: Span,
    ) -> Self {
        Self {
            kind,
            left_operand,
            right_operand,
            span,
        }
    }

    pub fn kind(&self) -> BinOpKind {
        self.kind.clone()
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn left_operand(&self) -> Box<AstNode> {
        self.left_operand.clone()
    }

    pub fn right_operand(&self) -> Box<AstNode> {
        self.right_operand.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnOpKind {
    Neg,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnOpNode {
    operand: Box<AstNode>,
    kind: UnOpKind,
    span: Span,
}

impl UnOpNode {
    pub fn new(kind: UnOpKind, operand: Box<AstNode>, span: Span) -> Self {
        Self {
            kind,
            operand,
            span,
        }
    }

    pub fn kind(&self) -> UnOpKind {
        self.kind.clone()
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn operand(&self) -> Box<AstNode> {
        self.operand.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FnCallNode {
    fn_name: String,
    args: Vec<Box<AstNode>>,
    span: Span,
}

impl FnCallNode {
    pub fn new(fn_name: String, span: Span) -> Self {
        Self {
            fn_name,
            args: Vec::new(),
            span,
        }
    }

    pub fn fn_name(&self) -> String {
        self.fn_name.clone()
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn args(&self) -> Vec<Box<AstNode>> {
        self.args.clone()
    }

    pub fn push_arg(&mut self, arg: Box<AstNode>) {
        self.args.push(arg);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AstNode {
    Num(NumNode),
    BinOp(BinOpNode),
    UnOp(UnOpNode),
    FnCall(FnCallNode),
}
