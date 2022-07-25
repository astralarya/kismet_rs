use crate::{ast::Atom, types::Node};

use super::{token_action, Input, KResult, NumberKind, Token};

pub fn atom<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    token_action(|x| match &*x.data {
        Token::Id(y) => Some(Node::new(x.span, Atom::Id(y.clone()))),
        Token::String(y) => Some(Node::new(x.span, Atom::String(y.clone()))),
        Token::Number(NumberKind::Integer(y)) => Some(Node::new(x.span, Atom::Integer(y.clone()))),
        Token::Number(NumberKind::Float(y)) => Some(Node::new(x.span, Atom::Float(y.clone()))),
        _ => None,
    })(i)
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
