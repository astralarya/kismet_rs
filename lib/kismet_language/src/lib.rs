mod ast;
mod lexer;

pub use ast::*;
pub use lexer::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub kismet);

pub fn parse<'input>(input: &'input String) -> ast::ParseResult<'input> {
    let lex = lexer::lex(input);
    kismet::KismetParser::new().parse(lex.into_iter())
}
