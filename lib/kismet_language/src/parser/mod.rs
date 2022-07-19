use nom::{error::Error, Err, Parser};

use crate::{ast::Node, types::Span};

mod token;
pub use token::Token;

pub type ParseResult<I, O> = Result<Node<O>, Err<Error<Node<I>>>>;

pub fn parse<'input>(input: &'input str) -> ParseResult<&'input str, Token<'input>> {
    run_parser(&mut token::delim, input)
}

pub fn run_parser<I, O, P>(parser: &mut P, input: I) -> ParseResult<I, O>
where
    P: Parser<Node<I>, Node<O>, Error<Node<I>>>,
    Span: From<I>,
    I: Copy,
{
    let (_, result) = parser.parse(Node::new(Span::from(input), input))?;
    Ok(result)
}
