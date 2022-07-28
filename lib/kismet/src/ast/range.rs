use core::fmt;

use crate::types::Node;

use super::Expr;

#[derive(Clone, Debug, PartialEq)]
pub enum Range {
    Range { start: Node<Expr>, end: Node<Expr> },
    RangeFrom { start: Node<Expr> },
    RangeTo { end: Node<Expr> },
    RangeFull,
    RangeI { start: Node<Expr>, end: Node<Expr> },
    RangeToI { end: Node<Expr> },
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Range { start, end } => write!(f, "{}..{}", start, end),
            Self::RangeFrom { start } => write!(f, "{}..", start),
            Self::RangeTo { end } => write!(f, "..{}", end),
            Self::RangeFull => write!(f, ".."),
            Self::RangeI { start, end } => write!(f, "{}..={}", start, end),
            Self::RangeToI { end } => write!(f, "..={}", end),
        }
    }
}
