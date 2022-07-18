use std::fmt;

use super::{Expr, Node, Target};

#[derive(Debug, PartialEq)]
pub enum CompIter<'input> {
    For {
        target: Vec<Node<Target<'input>>>,
        expr: Node<Expr<'input>>,
    },
    If(Node<Expr<'input>>),
}

impl fmt::Display for CompIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            CompIter::For { target, expr } => {
                write!(f, "for {} in {}", Node::vec_to_string(&target, ", "), expr)
            }
            CompIter::If(node) => write!(f, "if {}", node),
        }
    }
}
