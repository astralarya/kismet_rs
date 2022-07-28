use nom::{Err, IResult};

use crate::{
    ast::Expr,
    types::{Node, ONode, Span},
};

mod atom;
mod enclosure;
mod error;
mod expr;
mod primary;
mod stmt;
mod target;
mod token;

pub use atom::*;
pub use enclosure::*;
pub use error::*;
pub use expr::*;
pub use primary::*;
pub use stmt::*;
pub use target::*;
pub use token::*;

pub type Input<'a> = &'a [Node<Token>];
pub type Error = ONode<ErrorKind>;
pub type KResult<'a, O> = IResult<Input<'a>, O, Error>;

pub type ParseNode = Node<Expr>;

pub fn parse<'a>(input: &'a str) -> Result<ParseNode, Error> {
    run_parser(start, input)
}

pub fn run_parser<'a, P>(parser: P, i: &'a str) -> Result<ParseNode, Error>
where
    P: Fn(Input<'_>) -> KResult<'_, ParseNode>,
{
    let span = Span::from(i);
    let i = TokenIterator::new(i).collect::<Vec<_>>();
    let x = match parser(&i) {
        Ok((_, data)) => Ok(data),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(e),
        Err(Err::Incomplete(e)) => Err(ONode::new(
            Span::new(span.end..span.end),
            ErrorKind::Incomplete(e),
        )),
    };
    x
}
