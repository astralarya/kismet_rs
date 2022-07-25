use std::fmt;

use super::Expr;
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum DictItem {
    KeyDatum { key: Node<String>, val: Node<Expr> },
    Shorthand(String),
    Spread(Node<Expr>),
}

impl fmt::Display for DictItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            DictItem::KeyDatum { key, val } => write!(f, "{}: {}", key, val),
            DictItem::Shorthand(val) => write!(f, "{}", val),
            DictItem::Spread(val) => write!(f, "...{}", val),
        }
    }
}
