use std::fmt;

use super::{Expr, Node};

#[derive(Debug, PartialEq)]
pub enum KeyDatum<'input> {
    KeyDatum {
        key: Node<&'input str>,
        val: Node<Expr<'input>>,
    },
    Shorthand(&'input str),
    Spread(Node<Expr<'input>>),
}

impl fmt::Display for KeyDatum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            KeyDatum::KeyDatum { key, val } => write!(f, "{}: {}", key, val),
            KeyDatum::Shorthand(val) => write!(f, "{}", val),
            KeyDatum::Spread(val) => write!(f, "...{}", val),
        }
    }
}
