use std::fmt;

use crate::{ast::Id, types::Node};

#[derive(Clone, Debug, PartialEq)]
pub enum DictItem<T> {
    KeyVal { key: Node<Id>, val: Node<T> },
    DynKeyVal { key: Node<T>, val: Node<T> },
    Shorthand(Id),
    Spread(Node<T>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DictItemComp<T> {
    DynKeyVal { key: Node<T>, val: Node<T> },
    Spread(Node<T>),
}

impl<T: fmt::Display> fmt::Display for DictItem<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::KeyVal { key, val } => write!(f, "{}: {}", key, val),
            Self::DynKeyVal { key, val } => write!(f, "[{}]: {}", key, val),
            Self::Shorthand(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
        }
    }
}

impl<T: std::fmt::Display> fmt::Display for DictItemComp<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::DynKeyVal { key, val } => write!(f, "[{}]: {}", key, val),
            Self::Spread(val) => write!(f, "...{}", val),
        }
    }
}
