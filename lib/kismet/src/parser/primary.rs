use nom::{
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, tuple},
};

use crate::{
    ast::{Expr, Primary},
    types::Node,
};

use super::{atom, expr, token_tag, token_tag_id, Input, KResult, Token};

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
        let (i, val) = opt(attribute)(i)?;
        match val {
            Some(val) => {
                return Ok((
                    i,
                    Node::new(
                        iter.span.clone() + val.span.clone(),
                        Primary::Attribute(iter.clone(), val),
                    ),
                ))
            }
            None => (),
        }
        let (i, val) = opt(subscription)(i)?;
        match val {
            Some((_, val, rhs)) => {
                return Ok((
                    i,
                    Node::new(
                        iter.span.clone() + rhs.span.clone(),
                        Primary::Subscription(iter.clone(), val),
                    ),
                ))
            }
            None => (),
        }
        Ok((i, iter.clone()))
    }
}

pub fn attribute<'input>(i: Input<'input>) -> KResult<'input, Node<String>> {
    preceded(token_tag(Token::DOT), token_tag_id)(i)
}

pub fn subscription<'input>(
    i: Input<'input>,
) -> KResult<'input, (&Node<Token>, Vec<Node<Expr>>, &Node<Token>)> {
    tuple((
        token_tag(Token::LBRACKET),
        separated_list1(token_tag(Token::COMMA), expr),
        token_tag(Token::RBRACKET),
    ))(i)
}

pub fn primary_node<'input>(i: Input<'input>) -> KResult<'input, Node<Primary>> {
    let (i, val) = atom(i)?;
    Ok((i, Node::new(val.span.clone(), Primary::Atom(*val.data))))
}
