use std::convert::Infallible;

use crate::{ast, types::Node};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Never,
    Ast(Node<ast::Error>),
    TypeMismatch,
    //InvalidOp,
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Self::Never
    }
}

impl From<Node<ast::Error>> for Error {
    fn from(val: Node<ast::Error>) -> Self {
        Self::Ast(val)
    }
}
