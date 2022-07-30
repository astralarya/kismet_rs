use std::{fmt, ops::Deref};

use crate::types::Node;

use super::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct ExprTop(pub Vec<Node<Expr>>);

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBlock(pub Vec<Node<Expr>>);

#[derive(Clone, Debug, PartialEq)]
pub struct ExprEnclosure(pub Vec<Node<Expr>>);

impl Deref for ExprTop {
    type Target = Vec<Node<Expr>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ExprBlock {
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

impl fmt::Display for ExprBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            0 | 1 => write!(f, "{}", Node::join(self, "; ")),
            _ => write!(f, "{{ {} }}", Node::join(self, "; ")),
        }
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
