use std::fmt;

use super::Expr;
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum KeyDatum {
    KeyDatum { key: Node<String>, val: Node<Expr> },
    Shorthand(String),
    Spread(Node<Expr>),
}

impl fmt::Display for KeyDatum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            KeyDatum::KeyDatum { key, val } => write!(f, "{}: {}", key, val),
            KeyDatum::Shorthand(val) => write!(f, "{}", val),
            KeyDatum::Spread(val) => write!(f, "...{}", val),
        }
    }
}
