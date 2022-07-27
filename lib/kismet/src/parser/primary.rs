use std::collections::HashMap;

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    Err,
};

use crate::{
    ast::{Args, Expr, Primary},
    types::Node,
};

use super::{atom, expr, expr_as_id, token_tag, token_tag_id, Error, Input, KResult, Token};

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
    let assign = &token_tag(Token::ASSIGN);

    let (i, lhs) = open(i)?;

    let mut args: Vec<Node<Expr>> = vec![];
    let mut kwarg0_key: Option<Node<String>> = None;
    let mut i_ = i;
    let (i, args) = loop {
        let i = i_;
        let (i, arg) = opt(expr)(i)?;
        let arg = match arg {
            Some(arg) => arg,
            None => {
                let (i, rhs) = close(i)?;
                return Ok((
                    i,
                    Node::new(
                        lhs.span + rhs.span,
                        Args {
                            args,
                            kwargs: HashMap::new(),
                        },
                    ),
                ));
            }
        };
        let (i, sep) = opt(assign)(i)?;
        match sep {
            Some(_) => {
                let argspan = arg.span.clone();
                match expr_as_id(arg) {
                    Some(key) => {
                        kwarg0_key = Some(key);
                        break (i, args);
                    }
                    None => return Err(Err::Failure(Node::new(argspan, Error::Grammar))),
                }
            }
            None => args.push(arg),
        }
        let (i, sep) = opt(separator)(i)?;
        match sep {
            Some(_) => (),
            None => break (i, args),
        }
        i_ = i;
    };

    let mut kwargs: HashMap<String, Node<Expr>> = HashMap::new();
    let i = match kwarg0_key {
        Some(key) => {
            let (i, val) = expr(i)?;
            kwargs.insert(*key.data, val);
            i
        }
        None => i,
    };
    let (i, sep) = opt(separator)(i)?;
    match sep {
        Some(_) => (),
        None => {
            let (i, rhs) = close(i)?;
            return Ok((i, Node::new(lhs.span + rhs.span, Args { args, kwargs })));
        }
    }

    let mut i_ = i;
    let (i, kwargs) = loop {
        let i = i_;
        let (i, pair) = opt(separated_pair(token_tag_id, assign, expr))(i)?;
        match pair {
            Some((key, val)) => {
                kwargs.insert(*key.data, val);
            }
            None => break (i, kwargs),
        }
        let (i, sep) = opt(separator)(i)?;
        match sep {
            Some(_) => (),
            None => break (i, kwargs),
        }
        i_ = i
    };

    let (i, rhs) = close(i)?;

    Ok((i, Node::new(lhs.span + rhs.span, Args { args, kwargs })))
}

pub fn primary_node<'input>(i: Input<'input>) -> KResult<'input, Node<Primary>> {
    let (i, val) = atom(i)?;
    Ok((i, Node::new(val.span.clone(), Primary::Atom(*val.data))))
}
