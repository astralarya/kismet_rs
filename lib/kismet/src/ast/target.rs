use std::fmt;

use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum TargetList {
    Target(Target),
    List(Vec<Node<Target>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    Id(String),
    TargetTuple(Vec<Node<Target>>),
    TargetList(Vec<Node<Target>>),
    TargetDict(Vec<Node<TargetDictItem>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TargetDictItem {
    Shorthand(String),
    Pair {
        key: Node<String>,
        val: Node<Target>,
    },
}

impl fmt::Display for TargetList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetList::Target(val) => write!(f, "{}", val),
            TargetList::List(val) => match val.len() {
                1 => write!(f, "{},", Node::vec_to_string(val, ", ")),
                _ => write!(f, "{}", Node::vec_to_string(val, ", ")),
            },
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Id(val) => write!(f, "{}", val),
            Target::TargetTuple(val) => write!(f, "({})", Node::vec_to_string(&val, ", ")),
            Target::TargetList(val) => write!(f, "[{}]", Node::vec_to_string(&val, ", ")),
            Target::TargetDict(val) => write!(f, "{{{}}}", Node::vec_to_string(&val, ", ")),
        }
    }
}

impl fmt::Display for TargetDictItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetDictItem::Shorthand(val) => write!(f, "{}", val),
            TargetDictItem::Pair { key, val } => write!(f, "{}: {}", key, val),
        }
    }
}
