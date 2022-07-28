use std::fmt;

use super::{Expr, Target};
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum CompIter {
    For {
        target: Node<Target>,
        val: Node<Expr>,
    },
    If(Node<Expr>),
}

impl fmt::Display for CompIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::For { target, val } => {
                write!(f, "for {} in {}", target, val)
            }
            Self::If(val) => write!(f, "if {}", val),
        }
    }
}
