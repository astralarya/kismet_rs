use std::fmt;

use crate::ast::{Expr, Node};
use crate::token::Token;
use crate::types::{Integer, Span};

#[derive(Debug, PartialEq)]
pub enum Atom<'input> {
    Enclosure(Token<'input>, Node<Expr<'input>>, Token<'input>),
    Vector(Vec<Node<Expr<'input>>>),
    Tuple(Vec<Node<Expr<'input>>>),
    Id(&'input str),
    String(String),
    Integer(Integer),
}

impl<'input> Node<Atom<'input>> {
    pub fn enclosure(
        l: Token<'input>,
        n: Node<Expr<'input>>,
        r: Token<'input>,
    ) -> Node<Atom<'input>> {
        return Node {
            span: l.span.clone() + r.span.clone(),
            kind: Box::new(Atom::Enclosure(l, n, r)),
        };
    }

    pub fn vector((span, v): (Span, Vec<Node<Expr<'input>>>)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::Vector(v)),
        };
    }

    pub fn tuple((span, v): (Span, Vec<Node<Expr<'input>>>)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::Tuple(v)),
        };
    }

    pub fn id((span, string): (Span, &'input str)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::Id(string)),
        };
    }

    pub fn string((span, string): (Span, String)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::String(string)),
        };
    }

    pub fn integer((span, value): (Span, Integer)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::Integer(value)),
        };
    }
}

impl fmt::Display for Atom<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Atom::Enclosure(left, op, right) => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    left,
                    left.space(),
                    op,
                    right.space(),
                    right
                )
            }
            Atom::Vector(nodes) => write!(f, "[{}]", Node::vec_to_string(&nodes, ", ")),
            Atom::Tuple(nodes) => match nodes.len() {
                1 => write!(f, "({},)", nodes[0]),
                _ => write!(f, "({})", Node::vec_to_string(&nodes, ", ")),
            },
            Atom::String(s) => write!(f, r#""{}""#, s),
            Atom::Integer(n) => write!(f, "{}", n),
            Atom::Id(s) => write!(f, "{}", s),
        }
    }
}
