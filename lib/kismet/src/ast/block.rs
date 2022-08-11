use std::{fmt, ops::Deref};

use crate::{hir::VBasicBlock, types::Node};

use super::{Error, Expr};

#[derive(Clone, Debug, PartialEq)]
pub struct ExprTop(pub Vec<Node<Expr>>);

#[derive(Clone, Debug, PartialEq)]
pub struct ExprEnclosure(pub Vec<Node<Expr>>);

impl Deref for ExprTop {
    type Target = Vec<Node<Expr>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ExprEnclosure {
    type Target = Vec<Node<Expr>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ExprTop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Node::join(&self.0, "\n"))
    }
}

impl fmt::Display for ExprEnclosure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {} }}", Node::join(&self.0, "; "))
    }
}

impl TryFrom<ExprTop> for VBasicBlock {
    type Error = Error;

    fn try_from(val: ExprTop) -> Result<Self, Self::Error> {
        VBasicBlock::try_from(val.iter())
    }
}

impl TryFrom<ExprEnclosure> for VBasicBlock {
    type Error = Error;

    fn try_from(val: ExprEnclosure) -> Result<Self, Self::Error> {
        VBasicBlock::try_from(val.iter())
    }
}
