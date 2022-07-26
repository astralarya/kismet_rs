use std::fmt;

use super::{Atom, Expr};
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Primary {
    Attribute(Node<Primary>, Node<String>),
    Subscription(Node<Primary>, Vec<Node<Expr>>),
    Atom(Atom),
}

impl fmt::Display for Primary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primary::Attribute(lhs, rhs) => write!(f, "{}.{}", lhs, rhs),
            Primary::Subscription(lhs, rhs) => {
                write!(f, "{}[{}]", lhs, Node::vec_to_string(rhs, ", "))
            }
            Primary::Atom(val) => write!(f, "{}", val),
        }
    }
}
