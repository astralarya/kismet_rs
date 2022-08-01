use std::fmt;

use super::{Expr, Id};
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum DictItem {
    KeyVal { key: Node<Id>, val: Node<Expr> },
    DynKeyVal { key: Node<Expr>, val: Node<Expr> },
    Shorthand(Id),
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
            Self::KeyVal { key, val } => write!(f, "{}: {}", key, val),
            Self::DynKeyVal { key, val } => write!(f, "[{}]: {}", key, val),
            Self::Shorthand(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
        }
    }
}

impl fmt::Display for DictItemComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::DynKeyVal { key, val } => write!(f, "[{}]: {}", key, val),
            Self::Spread(val) => write!(f, "...{}", val),
        }
    }
}
