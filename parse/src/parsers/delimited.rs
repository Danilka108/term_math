use super::errors::*;
use super::parser::*;
use ir::span::*;
use ir::token::*;

impl Parser {
    fn parse_delimited_start(&mut self) -> PResult {
        let (delim_val, delim_span) = self.first().to_tuple();

        let delim_kind = match delim_val {
            Token::OpenDelim(delim_kind) => delim_kind,
            _ => return Ok(()),
        };

        matches_or!(
            self.second().to_tuple(),
            (Token::CloseDelim(close_delim_kind), _) if close_delim_kind == delim_kind,
            ERR__EMPTY_DELIMITED_BLOCK,
            delim_span,
        );

        self.push_buff_tuple(BuffElem::Delim(delim_kind), delim_span);
        Ok(())
    }

    fn parse_delimited_end(&mut self) -> Result<(), SpanWrapper<String>> {
        let (delim_val, delim_span) = self.first().to_tuple();

        let delim_kind = match delim_val {
            Token::CloseDelim(delim_kind) => delim_kind,
            _ => return Ok(()),
        };

        self.collect_ops()?;

        let (required_delim_val, _) =
            self.pop_buff_tuple_or(ERR__UNOPENED_DELIMITED_BLOCK, delim_span.clone())?;

        matches_or_else!(
            required_delim_val,
            BuffElem::Delim(required_delim_kind) if delim_kind == required_delim_kind,
            Ok(()),
            ERR__UNOPENED_DELIMITED_BLOCK,
            delim_span
        )
    }

    pub(crate) fn parse_delimited(&mut self) -> PResult {
        self.parse_delimited_start()?;
        self.parse_delimited_end()
    }
}
