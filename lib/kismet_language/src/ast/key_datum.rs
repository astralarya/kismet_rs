use std::fmt;

use super::{Expr, Node};

#[derive(Debug, PartialEq)]
pub enum KeyDatum<'input> {
    KeyDatum(Node<Expr<'input>>, Node<Expr<'input>>),
    Spread(Node<Expr<'input>>),
}

impl fmt::Display for KeyDatum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            KeyDatum::KeyDatum(l, r) => write!(f, "{}: {}", l, r),
            KeyDatum::Spread(v) => write!(f, "...{}", v),
        }
    }
}
