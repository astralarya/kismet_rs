use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, tuple},
};

use crate::{
    ast::{Args, Atom, Expr, Id, Primary},
    types::{Float, Node},
};

use super::{atom, expr, token_tag, token_tag_id, token_tag_idx, Input, KResult, Token};

pub fn primary(i: Input) -> KResult<Node<Primary>> {
    let (i, lhs) = opt(token_tag_idx)(i)?;
    let (mut i, mut iter) = match lhs {
        Some(lhs) => (
            i,
            Node::convert(
                |x| {
                    Primary::Atom(Atom::Float({
                        let x = x as Float;
                        let mag = x.log10().ceil();
                        x / mag
                    }))
                },
                lhs,
            ),
        ),
        None => primary_node(i)?,
    };
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
                Node::new(iter.span + val.span, Primary::Attribute(iter.clone(), val))
            }),
            map(tuple_index, |val| {
                Node::new(iter.span + val.span, Primary::Index(iter.clone(), val))
            }),
            map(subscription, |val| {
                Node::new(
                    iter.span + val.span,
                    Primary::Subscription(iter.clone(), *val.data),
                )
            }),
            map(call, |val| {
                Node::new(iter.span + val.span, Primary::Call(iter.clone(), val))
            }),
        ))(i)
    }
}

pub fn attribute(i: Input) -> KResult<Node<Id>> {
    preceded(token_tag(Token::DOT), token_tag_id)(i)
}

pub fn tuple_index(i: Input) -> KResult<Node<usize>> {
    token_tag_idx(i)
}

pub fn subscription(i: Input) -> KResult<Node<Vec<Node<Expr>>>> {
    map(
        tuple((
            token_tag(Token::LBRACKET),
            separated_list1(token_tag(Token::COMMA), expr),
            token_tag(Token::RBRACKET),
        )),
        |(lhs, val, rhs)| Node::new(lhs.span + rhs.span, val),
    )(i)
}

pub fn call(i: Input) -> KResult<Node<Args>> {
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

pub fn primary_node(i: Input) -> KResult<Node<Primary>> {
    let (i, val) = atom(i)?;
    Ok((i, Node::convert(Primary::Atom, val)))
}
