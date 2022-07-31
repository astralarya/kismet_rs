use std::fmt;

use super::Expr;
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Args(pub Vec<Node<Expr>>);

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Node::join(&self.0, ", "))
    }
}
