use super::errors::*;
use super::parser::*;
use ir::ast::*;
use ir::span::*;
use ir::token::*;

impl Parser {
    fn build_fn(
        &mut self,
        fn_name: String,
        args_count: usize,
        span: Span,
    ) -> Result<(), SpanWrapper<String>> {
        let mut args = Vec::new();

        for _ in 0..args_count {
            let arg = self.pop_node_or(ERR__MISSING_ARG, span.clone())?;
            args.push(arg);
        }

        args.reverse();

        let node = SpanWrapper::new(Node::Fn(fn_name, args), span);
        self.push_node(node);

        Ok(())
    }

    fn parse_fn_name(&mut self) -> PResult {
        let (ident_val, ident_span) = self.first().to_tuple();

        let fn_name = match ident_val {
            Token::Ident(val) => val,
            _ => return Ok(()),
        };

        matches_or!(
            self.second().to_tuple(),
            (Token::OpenDelim(DelimKind::Paren), _),
            ERR__MISSING_ARGS_BLOCK_START_PAREN,
            ident_span,
        );

        self.bump();
        self.bump();

        self.push_buff_tuple(BuffElem::Fn(fn_name, 0), ident_span);
        Ok(())
    }

    fn parse_fn_arg_separator(&mut self) -> Result<(), SpanWrapper<String>> {
        let separator_span = match self.first().val() {
            Token::Lit(LitKind::Comma) => self.first().span(),
            _ => return Ok(()),
        };

        matches_or!(
            self.second().to_tuple(),
            (
                Token::Lit(LitKind::Comma) | Token::CloseDelim(DelimKind::Paren),
                span
            ),
            ERR__EMPTY_ARG,
            [separator_span, span].concat_span()
        );

        self.collect_ops()?;

        matches_or_else!(
            self.mut_last_buff_tuple(),
            Some((buff_elem, buff_elem_span)),
            matches_or_else!(
                buff_elem,
                BuffElem::Fn(_, count),
                *count += 1,
                ERR__MISSING_ARGS_BLOCK_START,
                [&separator_span, &buff_elem_span].concat_span()
            ),
            ERR__MISSING_ARGS_BLOCK_START,
            [&separator_span, &Span::new(0, 0)].concat_span()
        );

        Ok(())
    }

    fn parse_fn_end(&mut self) -> Result<(), SpanWrapper<String>> {
        let (delim_val, delim_span) = self.first().to_tuple();

        match delim_val {
            Token::CloseDelim(DelimKind::Paren) => (),
            _ => return Ok(()),
        };

        self.collect_ops()?;

        match self.pop_buff_tuple() {
            Some((val, span)) => match val {
                BuffElem::Fn(fn_name, args_count) => {
                    self.bump();
                    self.build_fn(fn_name, args_count + 1, [span, delim_span].concat_span())
                }
                buff_elem => {
                    self.push_buff_tuple(buff_elem, span);
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }

    pub(crate) fn parse_fn(&mut self) -> Result<(), SpanWrapper<String>> {
        self.parse_fn_name()?;
        self.parse_fn_arg_separator()?;
        self.parse_fn_end()
    }
}
