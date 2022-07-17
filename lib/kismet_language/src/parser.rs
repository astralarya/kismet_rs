use lalrpop_util::ParseError as LalrpopError;

use crate::ast::{Expr, Node};
use crate::kismet::{self as lalrpop};
use crate::lexer::{lex, LexerError};
use crate::token::Token;

pub type ParseError<'input> = LalrpopError<usize, Token<'input>, LexerError>;
pub type ParseResult<'input> = Result<Node<Expr<'input>>, ParseError<'input>>;

pub fn parse<'input>(input: &'input str) -> ParseResult<'input> {
    KismetParser::new().parse(input)
}

trait Parser {
    fn do_parse<'input>(&self, input: &'input str) -> ParseResult<'input>;
}

macro_rules! declare_parser {
    ($parser: ident) => {
        pub struct $parser(lalrpop::$parser);

        impl $parser {
            fn new() -> Self {
                KismetParser(lalrpop::$parser::new())
            }

            fn parse<'input>(&self, input: &'input str) -> ParseResult<'input> {
                self.0.parse(lex(input).into_iter())
            }
        }
    };
}

declare_parser!(KismetParser);
