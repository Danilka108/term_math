const ADDITION_OPERATOR_PRIORITY: usize = 2;
const SUBTRACTION_OPERATOR_PRIORITY: usize = 2;
const DIVISION_OPERATOR_PRIORITY: usize = 1;
const MULTIPLICATION_OPERATOR_PRIORITY: usize = 1;

#[derive(Debug, Clone)]
pub enum OperatorKind {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}

#[derive(Debug, Clone)]
pub struct OperatorNode {
    left_operand: Option<Box<AstNode>>,
    right_operand: Option<Box<AstNode>>,
    kind: OperatorKind,
}

impl OperatorNode {
    pub fn new(kind: OperatorKind) -> Self {
        Self {
            left_operand: None,
            right_operand: None,
            kind,
        }
    }

    pub fn set_left_operand(mut self, operand: AstNode) -> Self {
        self.left_operand = Some(Box::new(operand));
        self
    }

    pub fn set_right_operand(mut self, operand: AstNode) -> Self {
        self.right_operand = Some(Box::new(operand));
        self
    }

    pub fn left_operand(&self) -> Option<Box<AstNode>> {
        self.left_operand.clone()
    }

    pub fn right_operand(&self) -> Option<Box<AstNode>> {
        self.right_operand.clone()
    }

    pub fn priority(&self) -> usize {
        use OperatorKind::*;

        match self.kind {
            Addition => ADDITION_OPERATOR_PRIORITY,
            Subtraction => SUBTRACTION_OPERATOR_PRIORITY,
            Multiplication => MULTIPLICATION_OPERATOR_PRIORITY,
            Division => DIVISION_OPERATOR_PRIORITY,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NumberNode {
    value: String,
}

impl NumberNode {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    function_name: String,
    args: Vec<AstNode>,
}

impl FunctionCallNode {
    pub fn new(function_name: String) -> Self {
        Self { function_name, args: Vec::new() }
    }

    pub fn push_arg(&mut self, arg: AstNode) {
        self.args.push(arg)
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Operator(OperatorNode),
    Number(NumberNode),
    FunctionCall(FunctionCallNode),
}
