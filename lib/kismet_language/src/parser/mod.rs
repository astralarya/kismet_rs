pub mod token;

use nom::IResult;

use token::Token;

use crate::{ast::Node, types::Span};

pub struct ParseError {}
//type ParseResult<'input> = Result<Node<Expr<'input>>, ParseError>;

pub fn parse<'input>(input: &'input str) -> IResult<Node<&'input str>, Node<Token<'input>>> {
    token::delim(Node::new(Span::from(input), input))
}
