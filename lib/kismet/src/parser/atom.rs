use crate::ast::{Atom };
use crate::types::Node;

use super::{token_action, KResult, NumberKind, Token};

pub fn atom<'input>(input: Node<&'input str>) -> KResult<Node<&'input str>, Node<Atom<'input>>> {
    token_action(|x| match *x.data {
        Token::Id(y) => Some(Node::new(x.span, Atom::Id(y))),
        Token::String(y) => Some(Node::new(x.span, Atom::String(y))),
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(y))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y))),
        _ => None,
    })(input)
}

pub fn id<'input>(input: Node<&'input str>) -> KResult<Node<&'input str>, Node<Atom<'input>>> {
    token_action(|x| match *x.data {
        Token::Id(y) => Some(Node::new(x.span, Atom::Id(y))),
        _ => None,
    })(input)
}

pub fn literal<'input>(input: Node<&'input str>) -> KResult<Node<&'input str>, Node<Atom<'input>>> {
    token_action(|x| match *x.data {
        Token::String(y) => Some(Node::new(x.span, Atom::String(y))),
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(y))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y))),
        _ => None,
    })(input)
}

pub fn string_literal<'input>(
    input: Node<&'input str>,
) -> KResult<Node<&'input str>, Node<Atom<'input>>> {
    token_action(|x| match *x.data {
        Token::String(y) => Some(Node::new(x.span, Atom::String(y))),
        _ => None,
    })(input)
}

pub fn numeric_literal<'input>(
    input: Node<&'input str>,
) -> KResult<Node<&'input str>, Node<Atom<'input>>> {
    token_action(|x| match *x.data {
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(y))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y))),
        _ => None,
    })(input)
}
