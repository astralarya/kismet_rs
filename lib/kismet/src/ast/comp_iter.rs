use std::fmt;

use super::{Expr, TargetList};
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum CompIter {
    For {
        target: Node<TargetList>,
        val: Node<Expr>,
    },
    If(Node<Expr>),
}

impl fmt::Display for CompIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            CompIter::For { target, val } => {
                write!(f, "for {} in {}", target, val)
            }
            CompIter::If(val) => write!(f, "if {}", val),
        }
    }
}
