macro_rules! unwrap_enum {
    ($unwraped:expr, $pat:path) => {
        match $unwraped {
            $pat(val) => val,
            _ => panic!("unsuccessful attempt to unwrap enum")
        } 
    };
}

macro_rules! consume {
    ($ex:expr) => {
        match $ex {
            Modification::Ok(ok) => return Modification::Ok(ok),
            Modification::Err(err) => return Modification::Err(err),
            Modification::None => ()
        }
    };

    ($cursor:expr, $consume:ident) => {
        match $consume($cursor.clone()) {
            Modification::Ok(ok) => return Modification::Ok(ok),
            Modification::Err(err) => return Modification::Err(err),
            Modification::None => ()
        }
    };

    ($cursor:expr, $consume:ident($($arg:expr)*)) => {
        match $consume($($arg)*)($cursor.clone()) {
            Modification::Err(err) => return Modification::Err(err),
            other => other,
        }
    }
}




mod delimited;
mod func;
mod num;
mod op;
mod unknown;

use ir::span::SpanWrapper;
use super::errors;
use op::*;
use func::*;
use delimited::*;
use ir::ast::{BinOpKind, Node, UnOpKind};
use ir::cursor::prelude::*;
use ir::cursor::TokenCursor;
use ir::token::DelimKind;

#[derive(Clone, Debug)]
enum ParseControlFlow<'expr> {
    Stop,
    New(Expr<'expr>)
}

type ParseRes<'expr> = Modification<ParseControlFlow<'expr>, SpanWrapper<String>>;
type ConsumeRes<D> = Modification<SpanWrapper<D>, SpanWrapper<String>>;
type ProduceRes<D> = Modification<D, String>;


#[derive(Clone, Debug)]
pub enum ExprKind {
    Global,
    FnArg,
    Delimited(DelimKind),
}

#[derive(Clone, Debug)]
pub struct Expr<'cursor> {
    kind: ExprKind,
    pub(self) cursor: TokenCursor<'cursor>,
    pub(self) buffer: Vec<SpanWrapper<BuffElement>>,
    pub(self) nodes: Vec<SpanWrapper<Node>>,
}

#[derive(Clone, Debug)]
enum BuffElement {
    Fn(String),
    BinOp(BinOpKind),
    UnOp(UnOpKind),
}

impl<'expr> Expr<'expr> {
    const PARSERS: [fn(&mut Self) -> ParseRes<'expr>; 5] = [
        Self::parse_fn,
        Self::parse_delimited,
        Self::parse_num,
        Self::parse_op,
        Self::parse_unknown,
    ];

    pub fn new(kind: ExprKind, cursor: TokenCursor<'expr>) -> Self {
        Self {
            cursor,
            kind,
            buffer: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn kind(&self) -> &ExprKind {
        &self.kind
    }

    pub fn attach_fn_args(&mut self, args: &mut Vec<SpanWrapper<Node>>) {
        let buff_elem = match self.buffer.pop() {
            Some(item) => item,
            None => return,
        };

        let (name, span) = match buff_elem.to_tuple() {
            (BuffElement::Fn(name), span) => (name, span),
            (val, span) => {
                self.buffer.push(SpanWrapper::new(val, span));
                return;
            },
        };

        let mut owned_args = Vec::new();
        while let Some(arg) = args.pop() {
            owned_args.push(arg);
        }

        let val = Node::Fn(name, owned_args);
        self.nodes.push(SpanWrapper::new(val, span));
    }

    pub fn attach_delimited(&mut self, delimited: Option<SpanWrapper<Node>>) {
        match delimited {
            Some(node) => {
                self.nodes.push(node)
            },
            _ => (),
        }
    }

    pub fn collect_nodes(mut self) -> SpanWrapper<Node> {
        while let Some(buff_item) = self.buffer.pop() {
            let (val, span) = buff_item.to_tuple();

            let node = match val {
                BuffElement::BinOp(op) => self.build_op(&op),
                BuffElement::UnOp(op) => self.build_op(&op),
                BuffElement::Fn(_) => panic!("All functions had to be collected by the parser"),
            };

            self.nodes.push(SpanWrapper::new(node, span));
        }

        match self.nodes.pop() {
            Some(node) => node,
            _ => panic!("After collecting, only one node should remain"),
        }
    }

    pub fn parse(&mut self) -> ParseRes<'expr> {
        while !self.cursor.is_eof() {
            for parse in Self::PARSERS {
                match parse(self) {
                    ParseRes::None => (),
                    other => return other,
                }
            }
        }

        ParseRes::None
    }
}
