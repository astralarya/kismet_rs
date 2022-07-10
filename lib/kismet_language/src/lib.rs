mod ast;
pub use ast::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub kismet);

pub fn parse<'input>(input: &'input String) -> ast::ParseResult<'input> {
    kismet::KismetParser::new().parse(&input)
}
