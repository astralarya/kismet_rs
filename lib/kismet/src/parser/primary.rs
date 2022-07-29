use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, tuple},
};

use crate::{
    ast::{Args, Expr, Primary},
    types::Node,
};

use super::{atom, expr, token_tag, token_tag_id, Input, KResult, Token};

pub fn primary<'input>(i: Input<'input>) -> KResult<'input, Node<Primary>> {
    let (mut i, mut iter) = primary_node(i)?;
    let mut next;
    loop {
        (i, next) = opt(primary_iter(iter.clone()))(i)?;
        match next {
            Some(next) => iter = next,
            None => return Ok((i, iter)),
        }
    }
}

pub fn primary_iter<'input>(
    iter: Node<Primary>,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<Primary>> {
    move |i| {
        alt((
            map(attribute, |val| {
                Node::new(
                    iter.span.clone() + val.span.clone(),
                    Primary::Attribute(iter.clone(), val),
                )
            }),
            map(subscription, |(lhs, val, rhs)| {
                Node::new(
                    lhs.span + rhs.span,
                    Primary::Subscription(iter.clone(), val),
                )
            }),
            map(call, |val| {
                Node::new(
                    iter.span.clone() + val.span.clone(),
                    Primary::Call(iter.clone(), val),
                )
            }),
        ))(i)
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

pub fn call<'input>(i: Input<'input>) -> KResult<'input, Node<Args>> {
    let open = &token_tag(Token::LPAREN);
    let close = &token_tag(Token::RPAREN);
    let separator = &token_tag(Token::COMMA);

    let (i, lhs) = open(i)?;

    let mut args: Vec<Node<Expr>> = vec![];
    let mut i_ = i;
    let (i, args) = loop {
        let i = i_;
        let (i, arg) = opt(expr)(i)?;
        match arg {
            Some(arg) => args.push(arg),
            None => break (i, args),
        };
        let (i, sep) = opt(separator)(i)?;
        match sep {
            Some(_) => (),
            None => break (i, args),
        }
        i_ = i;
    };
    let (i, rhs) = close(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Args(args))))
}

pub fn primary_node<'input>(i: Input<'input>) -> KResult<'input, Node<Primary>> {
    let (i, val) = atom(i)?;
    Ok((i, Node::new(val.span.clone(), Primary::Atom(*val.data))))
}
