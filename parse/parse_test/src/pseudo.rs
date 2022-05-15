#[parse_util(app: [OpModule])]
struct Parser {
    #[parse_util(emitter)]
    eof_emitter: Emitter<EofEvent>,

    #[parse_util(emitter)]
    parsing_emitter: Emitter<ParsingEvent>,

    #[parse_util(tokenizer)]
    tokenizer: Tokenizer<Token>,
}

impl ParserHooks for Parser {
    fn eof_hook(&mut self) -> PResult {
        self.eof_emitter.emit(EofEvent)?;
        Ok(())
    }

    fn parsing_hook(&mut self) -> PResult {
        self.parsing_emitter.emit(ParsingEvent)?;
        Ok(())
    }
}

#[parse_util(event)]
struct EofEvent;

#[parse_util(event)]
struct ParsingEvent;

#[parse_util(event)]
struct CollectingOpsEvent(bool);

struct Context<T: Clone + Debug, N, B> {
    tokenizer: Tokenizer<T>,
    state: State<N>,
    buffer: Buffer<B>,
}

struct OpModule {
    collect_ops_emitter: Emitter<CollectingOpsEvent>,
    context: Context<Token, Node, BinOpKind>,
}

impl OpModule {
    #![parse_util(module)]

    fn new(context: Context<Token, Node, BinOpKind>, collect_ops_emitter: Emitter<CollectOpsEvent>) -> Self {
        Self {
            context,
            collect_ops_emitter,
        }
    }

    #[parse_util(listener)]
    fn listen_eof(&mut self, _: EofEvent) -> PResult {
        self.collecting_ops_emitter.emit(CollectingOpsEvent(true))?;
        unimplemented!()
    }

    #[parse_util(listener)]
    fn listen_parsing(&mut self, _: ParsingEvent) -> PResult {
        unimplemented!()
    }
}
