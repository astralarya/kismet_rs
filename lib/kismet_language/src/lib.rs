use lalrpop_util::{lexer::Token, ParseError};

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub kismet);

pub fn parse<'input>(
    input: &'input String,
) -> Result<i32, ParseError<usize, Token<'input>, &'input str>> {
    kismet::TermParser::new().parse(&input)
}
