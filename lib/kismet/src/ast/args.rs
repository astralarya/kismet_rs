use std::fmt;

use super::{Expr, TargetExpr};
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Args(pub Vec<Node<Expr>>);

#[derive(Clone, Debug, PartialEq)]
pub struct ArgsDef(pub Vec<Node<TargetExpr>>);

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Node::join(&self.0, ", "))
    }
}

impl fmt::Display for ArgsDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Node::join(&self.0, ", "))
    }
}
