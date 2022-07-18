use std::fmt;

use crate::ast::{Expr, Node};

#[derive(Debug, PartialEq)]
pub enum SpreadItem<'input> {
    Expr(Expr<'input>),
    Spread(Node<Expr<'input>>),
}

impl fmt::Display for SpreadItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            SpreadItem::Expr(val) => write!(f, "{}", val),
            SpreadItem::Spread(val) => write!(f, "...{}", val),
        }
    }
}
