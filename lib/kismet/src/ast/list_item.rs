use std::fmt;

use crate::ast::Expr;
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum ListItem {
    Expr(Expr),
    Spread(Node<Expr>),
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            ListItem::Expr(val) => write!(f, "{}", val),
            ListItem::Spread(val) => write!(f, "...{}", val),
        }
    }
}
