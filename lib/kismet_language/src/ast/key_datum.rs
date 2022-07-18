use std::fmt;

use crate::types::Span;

use super::{Expr, Node};

#[derive(Debug, PartialEq)]
pub enum KeyDatum<'input> {
    KeyDatum(Node<Expr<'input>>, Node<Expr<'input>>),
    Spread(Node<Expr<'input>>),
}

impl<'input> Node<KeyDatum<'input>> {
    pub fn key_datum((span, value): (Span, KeyDatum<'input>)) -> Node<KeyDatum<'input>> {
        Node {
            span,
            kind: Box::new(value),
        }
    }
}

impl fmt::Display for KeyDatum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            KeyDatum::KeyDatum(l, r) => write!(f, "{}: {}", l, r),
            KeyDatum::Spread(n) => write!(f, "...{}", n),
        }
    }
}
