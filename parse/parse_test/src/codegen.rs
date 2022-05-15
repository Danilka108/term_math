use crate::context::*;
use crate::event::*;
use ir::ast::*;
use ir::span::{Span, SpanWrapper};
use ir::token::Token;
use std::cell::RefCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Parser__Tokenizer<T: Clone + Debug, TK: Tokenizer<T>> {
    tokenizer: Rc<RefCell<TK>>,
    _t: PhantomData<T>,
}

impl<T: Clone + Debug, TK: Tokenizer<T>> Parser__Tokenizer<T, TK> {
    fn new(tokenizer: Rc<RefCell<TK>>) -> Self {
        Self {
            tokenizer,
            _t: PhantomData,
        }
    }
}

impl<T: Clone + Debug, TK: Tokenizer<T>> Tokenizer<T> for Parser__Tokenizer<T, TK> {
    fn curr(&self) -> SpanWrapper<T> {
        let tokenizer = self.tokenizer.borrow();
        tokenizer.curr()
    }

    fn first(&self) -> SpanWrapper<T> {
        let tokenizer = self.tokenizer.borrow();
        tokenizer.first()
    }

    fn second(&self) -> SpanWrapper<T> {
        let tokenizer = self.tokenizer.borrow();
        tokenizer.second()
    }

    fn is_eof(&self) -> bool {
        let tokenizer = self.tokenizer.borrow();
        tokenizer.is_eof()
    }

    fn bump(&mut self) {
        let mut tokenizer = self.tokenizer.borrow_mut();
        tokenizer.bump();
    }
}

#[derive(Clone, Debug)]
struct Parser__NodesStack {
    nodes: Rc<RefCell<Vec<SpanWrapper<Node>>>>,
}

impl Parser__NodesStack {
    fn new(nodes: Rc<RefCell<Vec<SpanWrapper<Node>>>>) -> Self {
        Self { nodes }
    }
}

impl Stack<Node> for Parser__NodesStack {
    fn pop(&mut self) -> Option<SpanWrapper<Node>> {
        let mut nodes = self.nodes.borrow_mut();
        nodes.pop()
    }

    fn pop_tuple(&mut self) -> Option<(Node, Span)> {
        self.pop().map(|w| w.to_tuple())
    }

    fn push(&mut self, item: SpanWrapper<Node>) {
        let mut nodes = self.nodes.borrow_mut();
        nodes.push(item);
    }

    fn push_tuple(&mut self, val: Node, span: Span) {
        self.push(SpanWrapper::new(val, span));
    }
}

#[derive(Clone, Debug)]
enum Parser__Buffer {
    OpModule(BinOpKind),
}

#[derive(Clone, Debug)]
struct Parser__BufferStack {
    buffer: Vec<SpanWrapper<Parser__Buffer>>,
}

impl Default for Parser__BufferStack {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
        }
    }
}

impl Stack<Parser__Buffer> for Parser__BufferStack {
    fn pop_tuple(&mut self) -> Option<(Parser__Buffer, Span)> {
        self.buffer.pop().map(|w| w.to_tuple())
    }

    fn pop(&mut self) -> Option<SpanWrapper<Parser__Buffer>> {
        self.buffer.pop()
    }

    fn push_tuple(&mut self, val: Parser__Buffer, span: Span) {
        self.push(SpanWrapper::new(val, span));
    }

    fn push(&mut self, item: SpanWrapper<Parser__Buffer>) {
        self.buffer.push(item);
    }
}

#[derive(Clone, Debug)]
struct OpModule__BufferStack<MS: Stack<Parser__Buffer>> {
    main_buffer: Rc<RefCell<MS>>,
}

impl<MS: Stack<Parser__Buffer>> OpModule__BufferStack<MS> {
    fn new(main_buffer: Rc<RefCell<MS>>) -> Self {
        Self { main_buffer }
    }
}

impl<MS: Stack<Parser__Buffer>> Stack<BinOpKind> for OpModule__BufferStack<MS> {
    fn pop_tuple(&mut self) -> Option<(BinOpKind, Span)> {
        let mut buffer = self.main_buffer.borrow_mut();

        match buffer.pop() {
            Some(wrapper) => match wrapper.to_tuple() {
                (Parser__Buffer::OpModule(val), span) => Some((val, span)),
                (val, span) => {
                    buffer.push_tuple(val, span);
                    None
                }
            },
            None => None,
        }
    }

    fn pop(&mut self) -> Option<SpanWrapper<BinOpKind>> {
        self.pop_tuple().map(|(v, s)| SpanWrapper::new(v, s))
    }

    fn push_tuple(&mut self, val: BinOpKind, span: Span) {
        self.push(SpanWrapper::new(val, span));
    }

    fn push(&mut self, item: SpanWrapper<BinOpKind>) {
        let mut buffer = self.main_buffer.borrow_mut();
        buffer.push(item.map(|v| Parser__Buffer::OpModule(v)));
    }
}

pub fn parse<T: Tokenizer<Token> + 'static>(tokenizer: T) -> PResult<Box<SpanWrapper<Node>>> {
    let mut tokenizer = Parser__Tokenizer::new(Rc::new(RefCell::new(tokenizer)));
    let mut nodes_stack = Parser__NodesStack::new(Rc::new(RefCell::new(Vec::new())));
    let main_buffer_stack = Rc::new(RefCell::new(Parser__BufferStack::default()));

    let ParseEvent__event_manager = EventManager::default();
    let EofEvent__event_manager = EventManager::default();
    let CollectOpsEvent_event_manager = EventManager::default();

    OpModule::new(
        Context::new(
            tokenizer.clone(),
            OpModule__BufferStack::new(main_buffer_stack.clone()),
            nodes_stack.clone(),
        ),
        Emitter::new(CollectOpsEvent_event_manager.clone()),
    )
    .__attach_events(
        ParseEvent__event_manager.clone(),
        EofEvent__event_manager.clone(),
    );

    NumModule::new(
        Context::new(
            tokenizer.clone(),
            OpModule__BufferStack::new(main_buffer_stack.clone()),
            nodes_stack.clone(),
        ),
    ).__attach_events(
    ParseEvent__event_manager.clone(),
    EofEvent__event_manager.clone(),
    );

    let mut Parser = Parser::new(
        Emitter::new(EofEvent__event_manager),
        Emitter::new(ParseEvent__event_manager),
    );

    while !tokenizer.is_eof() {
        Parser.parsing_hook()?;
    }

    Parser.eof_hook()?;

    let ast = match nodes_stack.pop() {
        Some(node) => node,
        None => panic!(),
    };

    match nodes_stack.pop() {
        Some(_) => panic!(),
        None => (),
    }

    Ok(Box::new(ast))
}

//parse_util::build_parser! {
//  name: parse,
//  parser: Parser,
//  modules: [
//      OpModule,
//  ]
//}

//#[parse_util(parser)]
struct Parser {
    eof_emitter: Emitter<EofEvent>,
    parse_emitter: Emitter<ParseEvent>,
}

impl Parser {
    //#[parse_util(inject)]
    fn new(eof_emitter: Emitter<EofEvent>, parse_emitter: Emitter<ParseEvent>) -> Self {
        Self {
            eof_emitter,
            parse_emitter,
        }
    }
}

impl ParserHooks for Parser {
    fn parsing_hook(&mut self) -> PResult {
        self.parse_emitter.emit(ParseEvent)
    }

    fn eof_hook(&mut self) -> PResult {
        self.eof_emitter.emit(EofEvent)
    }
}

#[derive(Clone)]
struct ParseEvent;

#[derive(Clone)]
struct EofEvent;

#[derive(Clone)]
struct CollectOpsEvent;

#[derive(Clone)]
struct OpModule<T, B, N>
where
    T: Tokenizer<Token>,
    B: Stack<BinOpKind>,
    N: Stack<Node>,
{
    context: Context<Token, BinOpKind, Node, T, B, N>,
    collect_ops_emitter: Emitter<CollectOpsEvent>,
}

impl<T, B, N> OpModule<T, B, N>
where
    T: Tokenizer<Token> + 'static,
    B: Stack<BinOpKind> + 'static,
    N: Stack<Node> + 'static,
{
    //#![module]

    //#[inject]
    fn new(
        context: Context<Token, BinOpKind, Node, T, B, N>,
        collect_ops_emitter: Emitter<CollectOpsEvent>,
    ) -> Self {
        Self {
            context,
            collect_ops_emitter,
        }
    }

    fn __attach_events(
        self,
        parse_event_manager: EventManager<ParseEvent>,
        eof_event_manager: EventManager<EofEvent>,
    ) {
        let mut s = self.clone();
        parse_event_manager.listen(move |data| OpModule::listen_parse(&mut s, data));

        let mut s = self.clone();
        eof_event_manager.listen(move |data| OpModule::listen_eof(&mut s, data));
    }

    fn build_bin_op(&mut self, kind: BinOpKind, span: Span) -> PResult {
        let rhs = match self.context.nodes.pop() {
            Some(node) => node,
            None => return Err(SpanWrapper::new("missing right operand".to_owned(), span)),
        };

        let lhs = match self.context.nodes.pop() {
            Some(node) => node,
            None => return Err(SpanWrapper::new("missing left operand".to_owned(), span)),
        };

        let node = SpanWrapper::new(Node::BinOp(kind, Box::new(lhs), Box::new(rhs)), span);

        self.context.nodes.push(node);
        Ok(())
    }

    fn listen_collect_ops(&mut self) -> PResult {
        loop {
            let (val, span) = match self.context.buffer.pop_tuple() {
                Some(t) => t,
                None => break,
            };

            self.build_bin_op(val, span)?;
        }

        Ok(())
    }

    fn listen_parse(&mut self, data: ParseEvent) -> PResult {
        let (new_op_val, new_op_span) = self.context.tokenizer.first().to_tuple();

        use ir::token::LitKind;

        let new_op_kind = match new_op_val {
            Token::Lit(LitKind::Plus) => BinOpKind::Add,
            Token::Lit(LitKind::Hyphen) => BinOpKind::Sub,
            Token::Lit(LitKind::Asterisk) => BinOpKind::Mul,
            Token::Lit(LitKind::Slash) => BinOpKind::Div,
            _ => return Ok(()),
        };

        if is_empty(self.context.tokenizer.second().val()) {
            return Err(SpanWrapper::new(
                "missing right operand".to_owned(),
                new_op_span,
            ));
        }

        if !is_valid_right_operand(self.context.tokenizer.second().val()) {
            return Err(SpanWrapper::new(
                "invalid right operand".to_owned(),
                new_op_span,
            ));
        }

        if is_empty(self.context.tokenizer.curr().val()) {
            return Err(SpanWrapper::new(
                "missing left operand".to_owned(),
                new_op_span,
            ));
        }

        if !is_valid_left_operand(self.context.tokenizer.curr().val()) {
            return Err(SpanWrapper::new(
                "invalid left operand".to_owned(),
                new_op_span,
            ));
        }

        match self.context.buffer.pop_tuple() {
            Some((val, span)) => self.build_bin_op(val, span)?,
            None => (),
        }

        self.context.buffer.push_tuple(new_op_kind, new_op_span);
        self.context.tokenizer.bump();

        Ok(())
    }

    fn listen_eof(&mut self, data: EofEvent) -> PResult {
        self.listen_collect_ops()
    }
}

fn is_empty(tok: Token) -> bool {
    matches!(tok, Token::Eof)
}

fn is_valid_right_operand(tok: Token) -> bool {
    matches!(tok, Token::OpenDelim(_) | Token::Num(_) | Token::Ident(_))
}

fn is_valid_left_operand(tok: Token) -> bool {
    matches!(tok, Token::CloseDelim(_) | Token::Num(_))
}

#[derive(Clone)]
struct NumModule<T, B, N>
where
    T: Tokenizer<Token>,
    B: Stack<BinOpKind>,
    N: Stack<Node>,
{
    context: Context<Token, BinOpKind, Node, T, B, N>,
}

impl<T, B, N> NumModule<T, B, N>
where
    T: Tokenizer<Token> + 'static,
    B: Stack<BinOpKind> + 'static,
    N: Stack<Node> + 'static,
{
    //#![parse_util(module)]

    //#[parse_util(inject)]
    fn new(
        context: Context<Token, BinOpKind, Node, T, B, N>,
    ) -> Self {
        Self {
            context,
        }
    }

    fn __attach_events(
        self,
        parse_event_manager: EventManager<ParseEvent>,
        eof_event_manager: EventManager<EofEvent>,
    ) {
        let mut s = self.clone();
        parse_event_manager.listen(move |data| NumModule::listen_parse(&mut s, data));

        let mut s = self.clone();
        eof_event_manager.listen(move |data| NumModule::listen_eof(&mut s, data));
    }

    fn is_valid_num_bounds(&self) -> bool {
        use ir::token::LitKind;

        let is_valid_lhs = matches!(
            self.context.tokenizer.curr().val(),
            Token::Lit(
                LitKind::Asterisk
                    | LitKind::Slash
                    | LitKind::Plus
                    | LitKind::Hyphen
                    | LitKind::Comma
            ) | Token::OpenDelim(_)
                | Token::Eof
        );
        let is_valid_rhs = matches!(
            self.context.tokenizer.second().val(),
            Token::Lit(
                LitKind::Asterisk
                    | LitKind::Slash
                    | LitKind::Plus
                    | LitKind::Hyphen
                    | LitKind::Comma
            ) | Token::CloseDelim(_)
                | Token::Eof
        );

        let is_lhs_separator = matches!(self.context.tokenizer.curr().val(), Token::Eof);
        let is_rhs_separator = matches!(self.context.tokenizer.second().val(), Token::Eof);

        if is_lhs_separator && is_rhs_separator {
            return false;
        }

        is_valid_rhs && is_valid_lhs
    }

    fn listen_parse(&mut self, data: ParseEvent) -> PResult {
        let (token, span) = self.context.tokenizer.first().to_tuple();

        let val = match token {
            Token::Num(val) => val,
            _ => return Ok(()),
        };

        if !self.is_valid_num_bounds() {
            return Err(SpanWrapper::new(
                    "missing operator".to_owned(),
                    span,
                    ));
        }

        let node = SpanWrapper::new(Node::Num(val), span);
        self.context.nodes.push(node);

        self.context.tokenizer.bump();

        Ok(())
    }

    fn listen_eof(&mut self, data: EofEvent) -> PResult {
        Ok(())
    }
}
