use nom::Err;

use crate::{
    ast::Atom,
    types::{Node, Span},
};

use super::{Error, Input, KResult, NumberKind, Token};

pub fn atom<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    match i.get(0) {
        Some(x) => match *x.data.clone() {
            Token::Id(y) => Ok((&i[1..], Node::new(x.span, Atom::Id(y)))),
            Token::String(y) => Ok((&i[1..], Node::new(x.span, Atom::String(y)))),
            Token::Number(NumberKind::Integer(y)) => {
                Ok((&i[1..], Node::new(x.span, Atom::Integer(y))))
            }
            Token::Number(NumberKind::Float(y)) => Ok((&i[1..], Node::new(x.span, Atom::Float(y)))),
            Token::ERROR => Err(Err::Error(Node::new(Span::from_iter(i), Error::Lex))),
            _ => Err(Err::Error(Node::new(Span::from_iter(i), Error::Predicate))),
        },
        None => Err(Err::Error(Node::new(Span::from_iter(i), Error::Eof))),
    }
}

/*
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

 */
