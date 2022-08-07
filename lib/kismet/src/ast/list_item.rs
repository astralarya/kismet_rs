use std::fmt;

use crate::ast::Expr;
use crate::hlir::{self, VInstruction};
use crate::types::Node;

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

impl TryFrom<ListItem> for VInstruction {
    type Error = hlir::Error;

    fn try_from(val: ListItem) -> Result<Self, Self::Error> {
        match val {
            ListItem::Expr(x) => VInstruction::try_from(x),
            ListItem::Spread(_) => todo!(),
        }
    }
}
