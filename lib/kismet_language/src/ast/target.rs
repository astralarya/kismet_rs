use std::fmt;

use crate::types::Span;

use super::Node;

#[derive(Debug, PartialEq)]
pub enum Target<'input> {
    Id(&'input str),
    TargetTuple(Vec<Node<Target<'input>>>),
    TargetList(Vec<Node<Target<'input>>>),
}

impl<'input> Node<Target<'input>> {
    pub fn target(span: Span, val: Target<'input>) -> Node<Target<'input>> {
        Node {
            span,
            kind: Box::new(val),
        }
    }
}

impl fmt::Display for Target<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Id(val) => write!(f, "{}", val),
            Target::TargetTuple(val) => write!(f, "({})", Node::vec_to_string(&val, ", ")),
            Target::TargetList(val) => write!(f, "[{}]", Node::vec_to_string(&val, ", ")),
        }
    }
}
