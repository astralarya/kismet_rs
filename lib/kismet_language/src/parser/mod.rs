mod atom;
mod expr;
mod token;

use nom::{Err, IResult, Needed, Parser};

use crate::{
    ast::{Atom, Expr, Node},
    types::Span,
};

pub use atom::*;
pub use expr::*;
pub use token::*;

pub type KResult<I, O, E = Error<I>> = IResult<I, O, E>;

#[derive(Debug, PartialEq)]
pub struct Error<I> {
    pub input: I,
    pub code: ErrorKind,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Eof,
    Lex,
    Incomplete(Needed),
    Predicate,
}

pub fn parse<'input>(input: &'input str) -> Result<Node<Atom<'input>>, Error<Node<&'input str>>> {
    run_parser(&mut numeric_literal, input)
}

pub fn run_parser<I, O, P>(parser: &mut P, i: I) -> Result<O, Error<Node<I>>>
where
    P: Parser<Node<I>, O, Error<Node<I>>>,
    Span: From<I>,
    I: Copy,
{
    let input = Node::new(Span::from(i), i);
    match parser.parse(input.clone()) {
        Ok((_, result)) => Ok(result),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(e),
        Err(Err::Incomplete(e)) => Err(Error {
            input: input,
            code: ErrorKind::Incomplete(e),
        }),
    }
}
