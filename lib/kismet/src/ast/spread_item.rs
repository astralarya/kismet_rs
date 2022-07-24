use std::fmt;

use crate::ast::Expr;
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum SpreadItem {
    Expr(Expr),
    Spread(Node<Expr>),
}

impl fmt::Display for SpreadItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            SpreadItem::Expr(val) => write!(f, "{}", val),
            SpreadItem::Spread(val) => write!(f, "...{}", val),
        }
    }
}
