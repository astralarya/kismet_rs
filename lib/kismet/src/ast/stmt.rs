use std::fmt;

use crate::types::Node;

use super::{Expr, Target};

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Assign { tar: Node<Target>, val: Node<Expr> },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign { tar, val } => write!(f, "{} = {}", tar, val),
        }
    }
}
