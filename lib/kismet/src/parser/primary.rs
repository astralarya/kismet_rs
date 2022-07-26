use nom::branch::alt;

use crate::{ast::Primary, types::Node};

use super::{atom, Input, KResult};

pub fn primary<'input>(i: Input<'input>) -> KResult<'input, Node<Primary>> {
    primary_node(i)
}

pub fn primary_node<'input>(i: Input<'input>) -> KResult<'input, Node<Primary>> {
    let (i, val) = atom(i)?;
    Ok((i, Node::new(val.span.clone(), Primary::Atom(*val.data))))
}
