use nom::{combinator::all_consuming, Err};

use crate::{
    ast::ExprTop,
    types::{Node, ONode, Span},
};

use super::{stmt_block0, Error, ErrorKind, Input, KResult};

pub fn start(i: Input) -> KResult<Node<ExprTop>> {
    let i_span = match Span::get0(i) {
        Some(x) => x,
        None => return Err(Err::Failure(ONode::new(None, Error::Error(ErrorKind::Eof)))),
    };
    let (i, val) = all_consuming(stmt_block0)(i)?;
    match val {
        Some(val) => Ok((i, Node::convert(ExprTop, val))),
        None => Err(Err::Failure(ONode::new(
            i_span,
            Error::Error(ErrorKind::Eof),
        ))),
    }
}
