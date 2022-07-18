use std::fmt;

use crate::token::Token;
use crate::types::{Float, Integer, Span};

use super::{CompIter, Expr, KeyDatum, Node, SpreadItem};

#[derive(Debug, PartialEq)]
pub enum Atom<'input> {
    Enclosure(Token<'input>, Node<Expr<'input>>, Token<'input>),
    ListDisplay(Vec<Node<SpreadItem<'input>>>),
    ListComprehension {
        value: Node<Expr<'input>>,
        iter: Vec<Node<CompIter<'input>>>,
    },
    DictDisplay(Vec<Node<KeyDatum<'input>>>),
    DictComprehension {
        key: Node<Expr<'input>>,
        value: Node<Expr<'input>>,
        iter: Vec<Node<CompIter<'input>>>,
    },
    Tuple(Vec<Node<Expr<'input>>>),
    Id(&'input str),
    String(String),
    Float(Float),
    Integer(Integer),
}

impl<'input> Node<Atom<'input>> {
    pub fn atom(span: Span, value: Atom<'input>) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(value),
        };
    }

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

    pub fn list_display((span, v): (Span, Vec<Node<SpreadItem<'input>>>)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::ListDisplay(v)),
        };
    }

    pub fn dict_display((span, v): (Span, Vec<Node<KeyDatum<'input>>>)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::DictDisplay(v)),
        };
    }

    pub fn dict_comprehension(
        span: Span,
        key: Node<Expr<'input>>,
        value: Node<Expr<'input>>,
        iter: Vec<Node<CompIter<'input>>>,
    ) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::DictComprehension { key, value, iter }),
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

    pub fn float((span, value): (Span, Float)) -> Node<Atom<'input>> {
        return Node {
            span,
            kind: Box::new(Atom::Float(value)),
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
            Atom::ListDisplay(nodes) => write!(f, "[{}]", Node::vec_to_string(&nodes, ", ")),
            Atom::ListComprehension { value, iter } => {
                write!(f, "[{} {}]", value, Node::vec_to_string(&iter, " "))
            }
            Atom::DictDisplay(nodes) => write!(f, "{{{}}}", Node::vec_to_string(&nodes, ", ")),
            Atom::DictComprehension { key, value, iter } => {
                write!(
                    f,
                    "{{{}: {} {}}}",
                    key,
                    value,
                    Node::vec_to_string(&iter, ", ")
                )
            }
            Atom::Tuple(nodes) => match nodes.len() {
                1 => write!(f, "({},)", nodes[0]),
                _ => write!(f, "({})", Node::vec_to_string(&nodes, ", ")),
            },
            Atom::String(s) => write!(f, r#""{}""#, s),
            Atom::Float(n) => write!(f, "{}", n),
            Atom::Integer(n) => write!(f, "{}", n),
            Atom::Id(s) => write!(f, "{}", s),
        }
    }
}
