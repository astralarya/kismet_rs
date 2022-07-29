use nom::{combinator::all_consuming, Err};

use crate::{
    ast::ExprBlockTop,
    types::{Node, ONode, Span},
};

use super::{expr_block0, ErrorKind, Input, KResult};

pub fn start<'input>(i: Input<'input>) -> KResult<'input, Node<ExprBlockTop>> {
    let i_span = match Span::get0(i) {
        Some(x) => x,
        None => return Err(Err::Failure(ONode::new(None, ErrorKind::Eof))),
    };
    let (i, val) = all_consuming(expr_block0)(i)?;
    match val {
        Some(val) => Ok((i, Node::new(val.span, ExprBlockTop(*val.data)))),
        None => Err(Err::Failure(ONode::new(i_span, ErrorKind::Eof))),
    }
}
