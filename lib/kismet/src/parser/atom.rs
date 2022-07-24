use crate::{ast::Atom, types::Node};

use super::{token_action, Input, KResult, NumberKind, Token};

pub fn atom<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match x {
        Token::Id(y) => Some(Node::new(x.span, Atom::Id(y))),
        Token::String(y) => Some(Node::new(x.span, Atom::String(y))),
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(y))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y))),
        _ => None,
    })
}

pub fn id<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match x {
        Token::Id(y) => Some(Node::new(x.span, Atom::Id(y))),
        _ => None,
    })
}

pub fn literal<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match x {
        Token::String(y) => Some(Node::new(x.span, Atom::String(y))),
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(y))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y))),
        _ => None,
    })
}

pub fn string_literal<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match x {
        Token::String(y) => Some(Node::new(x.span, Atom::String(y))),
        _ => None,
    })
}

pub fn numeric_literal<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match x {
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(y))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y))),
        _ => None,
    })
}
