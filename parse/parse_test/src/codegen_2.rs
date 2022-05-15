use ir::ast::*;
use ir::token::*;
use parse_util::*;

#[derive(Clone)]
pub struct CollectOpsEvent;

impl Event for CollectOpsEvent {
    fn __id() -> &'static str {
        "CollectOpsEvent"
    }
}

#[derive(Clone)]
pub struct ParseEvent;

impl Event for ParseEvent {
    fn __id() -> &'static str {
        "ParseEvent"
    }
}

#[derive(Clone)]
pub struct EofEvent;

impl Event for EofEvent {
    fn __id() -> &'static str {
        "EofEvent"
    }
}


#[derive(Clone)]
struct OpComponent {
    buffer: Buffer<BinOpKind>,
    nodes: Nodes<Node>,
    tokenizer: Tokenizer<Token>,
    collect_ops_emitter: Emitter<CollectOpsEvent>,
}

//#[parse_util(component)]
impl OpComponent {
    fn __attach_component(
        tokenizer: Tokenizer<Token>,
        buffer: Buffer<BinOpKind>,
        nodes: Nodes<Node>,
        mut events_manager: EventsManager,
    ) {
        let component = Self::construct(buffer, nodes, tokenizer, events_manager.emitter());

        let mut c = component.clone();
        events_manager.listen(move |data| c.listen_parse(data), 2);

        let mut c = component.clone();
        events_manager.listen(move |data| c.listen_eof(data), 0);
    }

    //#[parse_util(constructor)]
    fn construct(
        buffer: Buffer<BinOpKind>,
        nodes: Nodes<Node>,
        tokenizer: Tokenizer<Token>,
        collect_ops_emitter: Emitter<CollectOpsEvent>,
    ) -> Self {
        Self {
            buffer,
            nodes,
            tokenizer,
            collect_ops_emitter,
        }
    }

    //#[parse_util(listener: 2)]
    fn listen_parse(&mut self, data: ParseEvent) -> EResult {
        Ok(())
    }

    //#[parse_util(listener: 0)]
    fn listen_eof(&mut self, data: EofEvent) -> EResult {
        Ok(())
    }
}
