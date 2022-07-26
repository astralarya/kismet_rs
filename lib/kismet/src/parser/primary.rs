use nom::{combinator::opt, sequence::preceded};

use crate::{ast::Primary, types::Node};

use super::{atom, token_tag, token_tag_id, Input, KResult, Token};

pub fn primary<'input>(i: Input<'input>) -> KResult<'input, Node<Primary>> {
    let (mut i, mut iter) = primary_node(i)?;
    let mut prev = i.len();
    loop {
        (i, iter) = primary_iter(iter)(i)?;
        if i.len() == prev {
            break;
        } else {
            prev = i.len()
        }
    }
    Ok((i, iter))
}

pub fn primary_iter<'input>(
    iter: Node<Primary>,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<Primary>> {
    move |i| {
        let (i, val) = opt(preceded(token_tag(Token::DOT), token_tag_id))(i)?;
        match val {
            Some(val) => Ok((
                i,
                Node::new(
                    iter.span.clone() + val.span.clone(),
                    Primary::Attribute(iter.clone(), val),
                ),
            )),
            None => Ok((i, iter.clone())),
        }
    }
}

pub fn primary_node<'input>(i: Input<'input>) -> KResult<'input, Node<Primary>> {
    let (i, val) = atom(i)?;
    Ok((i, Node::new(val.span.clone(), Primary::Atom(*val.data))))
}
