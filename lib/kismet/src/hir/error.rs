use std::convert::Infallible;

use crate::{ast, types::Node};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Never,
    Node(Node<Error>),
    Ast(ast::Error),
    TypeMismatch,
    //InvalidOp,
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

impl From<ast::Error> for Error {
    fn from(val: ast::Error) -> Self {
        Self::Ast(val)
    }
}
