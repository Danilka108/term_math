use crate::constants::{
    ADD_OP_PRIORITY, DIV_OP_PRIORITY, MUL_OP_PRIORITY, NEG_OP_PRIORITY, SUB_OP_PRIORITY,
};
use crate::span::Span;

#[derive(Clone, Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Clone, Debug)]
pub struct BinOpNode {
    left_operand: Option<Box<AstNode>>,
    right_operand: Option<Box<AstNode>>,
    kind: BinOpKind,
}

impl BinOpNode {
    pub fn new(kind: BinOpKind) -> OpNode {
        OpNode::Bin(Self {
            kind,
            left_operand: None,
            right_operand: None,
        })
    }

    pub fn to_op_node(self) -> OpNode {
        OpNode::Bin(self)
    }

    pub fn set_left_operand(mut self, operand: AstNode) -> Self {
        self.left_operand = Some(Box::new(operand));
        self
    }

    pub fn set_right_operand(mut self, operand: AstNode) -> Self {
        self.right_operand = Some(Box::new(operand));
        self
    }

    pub fn kind(&self) -> BinOpKind {
        self.kind.clone()
    }

    pub fn left_operand(&self) -> Option<Box<AstNode>> {
        self.left_operand.clone()
    }

    pub fn right_operand(&self) -> Option<Box<AstNode>> {
        self.right_operand.clone()
    }
}

#[derive(Clone, Debug)]
pub enum UnOpKind {
    Neg,
}

#[derive(Clone, Debug)]
pub struct UnOpNode {
    right_operand: Option<Box<AstNode>>,
    kind: UnOpKind,
}

impl UnOpNode {
    pub fn new(kind: UnOpKind) -> OpNode {
        OpNode::Un(Self {
            kind,
            right_operand: None,
        })
    }

    pub fn to_op_node(self) -> OpNode {
        OpNode::Un(self)
    }

    pub fn set_right_operand(mut self, operand: AstNode) -> Self {
        self.right_operand = Some(Box::new(operand));
        self
    }

    pub fn kind(&self) -> UnOpKind {
        self.kind.clone()
    }
}

#[derive(Debug, Clone)]
pub enum OpNode {
    Bin(BinOpNode),
    Un(UnOpNode),
}

impl OpNode {
    pub fn priority(&self) -> usize {
        match self {
            Self::Bin(bin_op_node) => match bin_op_node.kind() {
                BinOpKind::Add => ADD_OP_PRIORITY,
                BinOpKind::Sub => SUB_OP_PRIORITY,
                BinOpKind::Mul => MUL_OP_PRIORITY,
                BinOpKind::Div => DIV_OP_PRIORITY,
            },
            Self::Un(un_op_node) => match un_op_node.kind() {
                UnOpKind::Neg => NEG_OP_PRIORITY,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct NumNode {
    value: String,
}

impl NumNode {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FnCallNode {
    fn_name: String,
    args: Vec<AstNode>,
}

impl FnCallNode {
    pub fn new(fn_name: String) -> Self {
        Self {
            fn_name,
            args: Vec::new(),
        }
    }

    pub fn push_arg(&mut self, arg: AstNode) {
        self.args.push(arg)
    }
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Op(OpNode),
    Num(NumNode),
    FnCall(FnCallNode),
}

#[derive(Debug, Clone)]
pub struct AstNode {
    kind: NodeKind,
    span: Span,
}

impl AstNode {
    pub fn new(kind: NodeKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn kind(&self) -> NodeKind {
        self.kind.clone()
    }
}
