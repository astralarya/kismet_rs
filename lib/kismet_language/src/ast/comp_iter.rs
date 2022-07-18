use std::fmt;

use super::{Expr, Node, TargetList};

#[derive(Debug, PartialEq)]
pub enum CompIter<'input> {
    For {
        target: Node<TargetList<'input>>,
        val: Node<Expr<'input>>,
    },
    If(Node<Expr<'input>>),
}

impl fmt::Display for CompIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            CompIter::For { target, val } => {
                write!(f, "for {} in {}", target, val)
            }
            CompIter::If(val) => write!(f, "if {}", val),
        }
    }
}
