use std::fmt;

use crate::types::Node;

use super::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct Target(pub TargetKind<Target>);

#[derive(Clone, Debug, PartialEq)]
pub enum TargetExpr {
    Target(TargetKind<TargetExpr>),
    TargetExpr(Node<Target>, Node<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TargetKind<T> {
    Id(String),
    TargetTuple(Vec<Node<TargetListItem<T>>>),
    TargetList(Vec<Node<TargetListItem<T>>>),
    TargetDict(Vec<Node<TargetDictItem<T>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TargetListItem<T> {
    Spread(Node<T>),
    Target(T),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TargetDictItem<T> {
    Pair { key: Node<String>, val: Node<T> },
    Spread(Node<T>),
    Target(String),
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TargetExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(tar) => write!(f, "{}", tar),
            Self::TargetExpr(tar, val) => write!(f, "{} = {}", tar, val),
        }
    }
}

impl<T> fmt::Display for TargetKind<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(val) => write!(f, "{}", val),
            Self::TargetTuple(val) => write!(f, "({})", Node::join1(&val, ", ", ",")),
            Self::TargetList(val) => write!(f, "[{}]", Node::join(&val, ", ")),
            Self::TargetDict(val) => write!(f, "{{{}}}", Node::join(&val, ", ")),
        }
    }
}

impl<T> fmt::Display for TargetListItem<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
        }
    }
}

impl<T> fmt::Display for TargetDictItem<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
            Self::Pair { key, val } => write!(f, "{}: {}", key, val),
        }
    }
}
