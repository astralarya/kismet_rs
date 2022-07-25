use nom::{branch::alt, combinator::opt};

use crate::{ast::Atom, types::Node};

use super::{expr, token_action, token_tag, Input, KResult, NumberKind, Token};

pub fn atom<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    alt((
        token_action(|x| match &*x.data {
            Token::Id(y) => Some(Node::new(x.span, Atom::Id(y.clone()))),
            Token::String(y) => Some(Node::new(x.span, Atom::String(y.clone()))),
            Token::Number(NumberKind::Integer(y)) => {
                Some(Node::new(x.span, Atom::Integer(y.clone())))
            }
            Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y.clone()))),
            _ => None,
        }),
        parentheses,
    ))(i)
}

pub fn parentheses<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    let (i, lhs) = token_tag(Token::LPAREN)(i)?;
    let (i, rhs) = opt(token_tag(Token::RPAREN))(i)?;
    match rhs {
        Some(rhs) => return Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(vec![])))),
        None => (),
    };
    let (i, val) = expr(i)?;
    let (i, rhs) = opt(token_tag(Token::RPAREN))(i)?;
    match rhs {
        Some(rhs) => return Ok((i, Node::new(lhs.span + rhs.span, Atom::Parentheses(val)))),
        None => (),
    }
    let (i, _) = token_tag(Token::COMMA)(i)?;
    let mut items = vec![val];
    let mut i = i;
    loop {
        let (_i, val) = opt(expr)(i)?;
        i = _i;
        match val {
            Some(val) => items.push(val),
            None => break,
        }
        let (_i, sep) = opt(token_tag(Token::COMMA))(i)?;
        i = _i;
        match sep {
            Some(_) => (),
            None => break,
        }
    }
    let (i, rhs) = token_tag(Token::RPAREN)(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(items))))
}

pub fn id<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match &*x.data {
        Token::Id(y) => Some(Node::new(x.span, Atom::Id(y.clone()))),
        _ => None,
    })(i)
}

pub fn literal<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match &*x.data {
        Token::String(y) => Some(Node::new(x.span, Atom::String(y.clone()))),
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(y.clone()))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y.clone()))),
        _ => None,
    })(i)
}

pub fn string_literal<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match &*x.data {
        Token::String(y) => Some(Node::new(x.span, Atom::String(y.clone()))),
        _ => None,
    })(i)
}

pub fn numeric_literal<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match &*x.data {
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(y.clone()))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y.clone()))),
        _ => None,
    })(i)
}
