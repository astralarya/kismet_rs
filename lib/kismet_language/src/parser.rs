use lalrpop_util::ParseError as LalrpopError;

use crate::ast::Node;
use crate::kismet::{self as lalrpop, KismetParser};
use crate::lexer::{lex, LexerError};
use crate::token::Token;

pub type ParseError<'input> = LalrpopError<usize, Token<'input>, LexerError>;
pub type ParseResult<'input> = Result<Node<'input>, ParseError<'input>>;

pub fn parse<'input>(input: &'input str) -> ParseResult<'input> {
    lalrpop::KismetParser::new().do_parse(input)
}

trait Parser {
    fn do_parse<'input>(&self, input: &'input str) -> ParseResult<'input>;
}

impl Parser for KismetParser {
    fn do_parse<'input>(&self, input: &'input str) -> ParseResult<'input> {
        self.parse(lex(input).into_iter())
    }
}
