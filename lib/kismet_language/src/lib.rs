use lalrpop_util::{lexer::Token, ParseError};
mod ast;
pub use ast::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub kismet);

pub fn parse<'input>(
    input: &'input String,
) -> Result<ast::Expr, ParseError<usize, Token<'input>, &'input str>> {
    kismet::KismetParser::new().parse(&input)
}
