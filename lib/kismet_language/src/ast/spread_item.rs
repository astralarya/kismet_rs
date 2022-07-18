use std::fmt;

use crate::types::Span;

use crate::ast::{Expr, Node};

#[derive(Debug, PartialEq)]
pub enum SpreadItem<'input> {
    Expr(Expr<'input>),
    Spread(Node<Expr<'input>>),
}

impl<'input> Node<SpreadItem<'input>> {
    pub fn spread_item((span, value): (Span, SpreadItem<'input>)) -> Node<SpreadItem<'input>> {
        Node {
            span,
            kind: Box::new(value),
        }
    }
}

impl fmt::Display for SpreadItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            SpreadItem::Expr(e) => write!(f, "{}", e),
            SpreadItem::Spread(n) => write!(f, "...{}", n),
        }
    }
}
