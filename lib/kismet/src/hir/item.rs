use std::fmt;

use crate::{
    ast::{self, Expr, Id},
    types::Node,
};

use super::Instruction;

#[derive(Clone, Debug, PartialEq)]
pub enum ListItem<T> {
    Expr(T),
    Spread(T),
}

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

impl<T: fmt::Display> fmt::Display for ListItem<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Expr(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
        }
    }
}

impl TryFrom<Node<ListItem<Expr>>> for Node<ListItem<Instruction>> {
    type Error = Node<ast::Error>;

    fn try_from(val: Node<ListItem<Expr>>) -> Result<Self, Self::Error> {
        Node::try_convert(
            |x| match x {
                ListItem::Expr(x) => Ok(ListItem::Expr(Instruction::try_from(x)?)),
                ListItem::Spread(x) => Ok(ListItem::Spread(Instruction::try_from(x)?)),
            },
            val,
        )
    }
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
