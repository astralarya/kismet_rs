pub mod ast;
pub mod lexer;
pub mod token;
pub mod types;

pub use ast::*;
pub use lexer::*;
pub use token::*;
pub use types::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub kismet);

pub fn parse<'input>(input: &'input str) -> ast::ParseResult<'input> {
    kismet::KismetParser::new().parse(lexer::lex(input).into_iter())
}
