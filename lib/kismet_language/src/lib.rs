mod ast;
mod lexer;
mod token;

pub use ast::*;
pub use lexer::*;
pub use token::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub kismet);

pub fn parse<'input>(input: &'input String) -> ast::ParseResult<'input> {
    kismet::KismetParser::new().parse(lexer::lex(input).into_iter())
}
