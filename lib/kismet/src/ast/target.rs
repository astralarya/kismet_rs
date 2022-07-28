use std::fmt;

use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    Id(String),
    TargetTuple(Vec<Node<TargetListItem>>),
    TargetList(Vec<Node<TargetListItem>>),
    TargetDict(Vec<Node<TargetDictItem>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TargetListItem {
    Spread(Node<Target>),
    Target(Target),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TargetDictItem {
    Pair {
        key: Node<String>,
        val: Node<Target>,
    },
    Spread(Node<Target>),
    Target(String),
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(val) => write!(f, "{}", val),
            Self::TargetTuple(val) => write!(f, "({})", Node::vec_to_string1(&val, ", ", ",")),
            Self::TargetList(val) => write!(f, "[{}]", Node::vec_to_string(&val, ", ")),
            Self::TargetDict(val) => write!(f, "{{{}}}", Node::vec_to_string(&val, ", ")),
        }
    }
}

impl fmt::Display for TargetListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
        }
    }
}

impl fmt::Display for TargetDictItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Target(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
            Self::Pair { key, val } => write!(f, "{}: {}", key, val),
        }
    }
}
