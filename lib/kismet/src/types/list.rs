use std::{fmt, ops::Deref};

use super::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct CommaList<T>(pub Vec<Node<T>>);

impl<T> Deref for CommaList<T> {
    type Target = Vec<Node<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> fmt::Display for CommaList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Node::join(&self.0, ", "))
    }
}
