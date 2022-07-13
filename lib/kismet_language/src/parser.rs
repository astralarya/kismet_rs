use crate::ast;
use crate::kismet;
use crate::lexer;

pub fn parse<'input>(input: &'input str) -> ast::ParseResult<'input> {
    let lex = lexer::lex(input);
    kismet::KismetParser::new().parse(lex.into_iter())
}
