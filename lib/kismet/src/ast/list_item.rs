use std::fmt;

use crate::ast::Expr;
use crate::hir::{ListItemKind, VInstruction};
use crate::types::Node;

use super::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum ListItem {
    Expr(Expr),
    Spread(Node<Expr>),
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Expr(val) => write!(f, "{}", val),
            Self::Spread(val) => write!(f, "...{}", val),
        }
    }
}

impl TryFrom<Node<ListItem>> for Node<(ListItemKind, VInstruction)> {
    type Error = Node<Error>;

    fn try_from(val: Node<ListItem>) -> Result<Self, Self::Error> {
        Node::try_convert(
            |x| match x {
                ListItem::Expr(x) => Ok((ListItemKind::Expr, VInstruction::try_from(x)?)),
                ListItem::Spread(x) => Ok((ListItemKind::Spread, VInstruction::try_from(*x.data)?)),
            },
            val,
        )
    }
}
