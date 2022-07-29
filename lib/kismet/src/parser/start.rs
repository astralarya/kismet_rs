use nom::{combinator::all_consuming, Err};

use crate::{
    ast::ExprBlock,
    types::{Node, ONode, Span},
};

use super::{expr_list0, ErrorKind, Input, KResult};

pub fn start<'input>(i: Input<'input>) -> KResult<'input, Node<ExprBlock>> {
    let i_span = match Span::get0(i) {
        Some(x) => x,
        None => return Err(Err::Failure(ONode::new(None, ErrorKind::Eof))),
    };
    let (i, val) = all_consuming(expr_list0)(i)?;
    match val {
        Some(val) => Ok((i, val)),
        None => Err(Err::Failure(ONode::new(i_span, ErrorKind::Eof))),
    }
}
