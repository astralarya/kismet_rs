use lalrpop_util::{lexer::Token, ParseError};
mod ast;
pub use ast::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub kismet);

type ParseResult<'life> = Result<ast::Expr, ParseError<usize, Token<'life>, &'life str>>;

pub fn parse<'input>(input: &'input String) -> ParseResult<'input> {
    kismet::KismetParser::new().parse(&input)
}
