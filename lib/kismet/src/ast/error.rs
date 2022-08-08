use std::convert::Infallible;

use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Never,
    Node(Node<Error>),
    Vec(Vec<Node<Error>>),
    TypeMismatch,
    InvalidOp,
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Self::Never
    }
}

impl From<Node<Error>> for Error {
    fn from(val: Node<Error>) -> Self {
        Self::Node(val)
    }
}

impl TryFrom<Error> for Node<Error> {
    type Error = ();

    fn try_from(val: Error) -> Result<Self, Self::Error> {
        match val {
            Error::Node(val) => Ok(val),
            _ => Err(()),
        }
    }
}
