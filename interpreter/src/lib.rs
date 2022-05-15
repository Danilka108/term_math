use ir::ast::*;
use ir::span::*;
use number::Dec64;


pub fn traverse_tree(ast: Box<SpanWrapper<Node>>) {

    let mut history = vec![ast];

    loop {
        let node = match history.pop() {
            Some(v) => v,
            None => break,
        };

        let number = match node.borrow_val() {
            Node::Num(n) => n,
            Node::BinOp(bin_op_kind, lhs, rhs) => {
                history.push(lhs.clone());
                history.push(rhs.clone());
                continue;
            },
            Node::UnOp(un_op_kind, operand) => {
                history.push(operand.clone());
                continue;
            }
            Node::Fn(_, _) => {
                todo!()
            },
        };

        break;
    }
}
