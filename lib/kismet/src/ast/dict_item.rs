use std::fmt;

use super::Expr;
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum DictItem {
    KeyVal { key: Node<String>, val: Node<Expr> },
    DynKeyVal { key: Node<Expr>, val: Node<Expr> },
    Shorthand(String),
    Spread(Node<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DictItemComp {
    DynKeyVal { key: Node<Expr>, val: Node<Expr> },
    Spread(Node<Expr>),
}

impl fmt::Display for DictItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            DictItem::KeyVal { key, val } => write!(f, "{}: {}", key, val),
            DictItem::DynKeyVal { key, val } => write!(f, "[{}]: {}", key, val),
            DictItem::Shorthand(val) => write!(f, "{}", val),
            DictItem::Spread(val) => write!(f, "...{}", val),
        }
    }
}

impl fmt::Display for DictItemComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            DictItemComp::DynKeyVal { key, val } => write!(f, "[{}]: {}", key, val),
            DictItemComp::Spread(val) => write!(f, "...{}", val),
        }
    }
}
