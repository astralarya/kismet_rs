mod atom;
mod error;
mod expr;
mod stmt;
mod token;

use nom::{Err, IResult, Parser};

use crate::{
    ast::{Expr, Node},
    types::Span,
};

pub use atom::*;
pub use error::*;
pub use expr::*;
pub use stmt::*;
pub use token::*;

pub type KResult<I, O, E = Error<I>> = IResult<I, O, E>;

pub fn parse<'input>(input: &'input str) -> Result<Node<Expr<'input>>, Error<Node<&'input str>>> {
    run_parser(&mut stmts, input)
}

pub fn run_parser<I, O, P>(parser: &mut P, i: I) -> Result<O, Error<Node<I>>>
where
    P: Parser<Node<I>, O, Error<Node<I>>>,
    Span: From<I>,
    I: Copy,
{
    let input = Node::from(i);
    match parser.parse(input.clone()) {
        Ok((_, result)) => Ok(result),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(e),
        Err(Err::Incomplete(e)) => Err(Error {
            input,
            code: ErrorKind::Incomplete(e),
        }),
    }
}
