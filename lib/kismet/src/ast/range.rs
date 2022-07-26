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
            Range::Range { start, end } => write!(f, "{}..{}", start, end),
            Range::RangeFrom { start } => write!(f, "{}..", start),
            Range::RangeTo { end } => write!(f, "..{}", end),
            Range::RangeFull => write!(f, ".."),
            Range::RangeI { start, end } => write!(f, "{}..={}", start, end),
            Range::RangeToI { end } => write!(f, "..={}", end),
        }
    }
}
