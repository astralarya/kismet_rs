use std::fmt;

use super::Node;

#[derive(Debug, PartialEq)]
pub enum Target<'input> {
    Id(&'input str),
    TargetTuple(Vec<Node<Target<'input>>>),
    TargetList(Vec<Node<Target<'input>>>),
    TargetDict(Vec<Node<TargetDictItem<'input>>>),
}

#[derive(Debug, PartialEq)]
pub enum TargetDictItem<'input> {
    Shorthand(&'input str),
    Pair {
        key: Node<&'input str>,
        val: Node<Target<'input>>,
    },
}

impl fmt::Display for Target<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Id(val) => write!(f, "{}", val),
            Target::TargetTuple(val) => write!(f, "({})", Node::vec_to_string(&val, ", ")),
            Target::TargetList(val) => write!(f, "[{}]", Node::vec_to_string(&val, ", ")),
            Target::TargetDict(val) => write!(f, "{{{}}}", Node::vec_to_string(&val, ", ")),
        }
    }
}

impl fmt::Display for TargetDictItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetDictItem::Shorthand(val) => write!(f, "{}", val),
            TargetDictItem::Pair { key, val } => write!(f, "{}: {}", key, val),
        }
    }
}
