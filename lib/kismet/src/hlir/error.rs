use std::convert::Infallible;

use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Never,
    TypeMismatch,
    InvalidOp,
    List(Node<Error>),
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Self::Never
    }
}
