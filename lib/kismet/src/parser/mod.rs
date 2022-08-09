use nom::{Err, IResult};

use crate::{
    ast::ExprTop,
    types::{Node, ONode, Span},
};

mod atom;
mod enclosure;
mod error;
mod expr;
mod op;
mod primary;
mod start;
mod stmt;
mod target;
mod token;

pub use atom::*;
pub use enclosure::*;
pub use error::*;
pub use expr::*;
pub use op::*;
pub use primary::*;
pub use start::*;
pub use stmt::*;
pub use target::*;
pub use token::*;

pub type Input<'a> = &'a [Node<Token>];
pub type KResult<'a, O> = IResult<Input<'a>, O, ONode<Error<'a>>>;

pub type ParseNode = Node<ExprTop>;

pub fn parse(input: &str) -> Result<ParseNode, ONode<ErrorKind>> {
    run_parser(start, input)
}

pub fn run_parser<P>(parser: P, i: &str) -> Result<ParseNode, ONode<ErrorKind>>
where
    P: Fn(Input<'_>) -> KResult<'_, ParseNode>,
{
    let span = Span::from(i);
    let i = TokenIterator::new(i).collect::<Vec<_>>();
    match parser(&i) {
        Ok((_, data)) => Ok(data),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(ONode::<ErrorKind>::convert_from(e)),
        Err(Err::Incomplete(e)) => Err(ONode::new(
            Span::new(span.end..span.end),
            ErrorKind::Incomplete(e),
        )),
    }
}
