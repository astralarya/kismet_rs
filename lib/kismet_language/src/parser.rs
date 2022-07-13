use super::ast;
use super::kismet;
use super::lexer;

pub fn parse<'input>(input: &'input str) -> ast::ParseResult<'input> {
    kismet::KismetParser::new().parse(lexer::lex(input).into_iter())
}
