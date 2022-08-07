use nom::branch::alt;

use crate::{ast::Atom, types::Node};

use super::{enclosure, token_action, Input, KResult, NumberKind, Token};

pub fn atom(i: Input) -> KResult<Node<Atom>> {
    alt((
        token_action(|x| match &*x.data {
            Token::Id(y) => Some(Node::new(x.span, Atom::Id(y.clone()))),
            Token::String(y) => Some(Node::new(x.span, Atom::String(y.clone()))),
            Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(*y))),
            Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(*y))),
            _ => None,
        }),
        enclosure,
    ))(i)
}

pub fn id(i: Input) -> KResult<Node<Atom>> {
    token_action(|x| match &*x.data {
        Token::Id(y) => Some(Node::new(x.span, Atom::Id(y.clone()))),
        _ => None,
    })(i)
}

pub fn literal(i: Input) -> KResult<Node<Atom>> {
    token_action(|x| match &*x.data {
        Token::String(y) => Some(Node::new(x.span, Atom::String(y.clone()))),
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(*y))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(*y))),
        _ => None,
    })(i)
}

pub fn string_literal(i: Input) -> KResult<Node<Atom>> {
    token_action(|x| match &*x.data {
        Token::String(y) => Some(Node::new(x.span, Atom::String(y.clone()))),
        _ => None,
    })(i)
}

pub fn numeric_literal(i: Input) -> KResult<Node<Atom>> {
    token_action(|x| match &*x.data {
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(*y))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(*y))),
        _ => None,
    })(i)
}
