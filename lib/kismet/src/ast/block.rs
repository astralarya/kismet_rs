use std::{fmt, ops::Deref};

use crate::types::Node;

use super::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBlock(pub Vec<Node<Expr>>);

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBlockTop(pub Vec<Node<Expr>>);

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBlockEnclosed(pub Vec<Node<Expr>>);

impl Deref for ExprBlock {
    type Target = Vec<Node<Expr>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ExprBlockTop {
    type Target = Vec<Node<Expr>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ExprBlockEnclosed {
    type Target = Vec<Node<Expr>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ExprBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            0 | 1 => write!(f, "{}", Node::join(self, "\n")),
            _ => write!(f, "{{\n{}\n}}", Node::join(self, "\n")),
        }
    }
}

impl fmt::Display for ExprBlockTop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Node::join(&self.0, "\n"))
    }
}

impl fmt::Display for ExprBlockEnclosed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n{}\n}}", Node::join(&self.0, "\n"))
    }
}
