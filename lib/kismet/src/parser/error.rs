use nom::{error::ParseError, Needed};

use crate::{
    ast::TargetExpr,
    types::{Node, ONode, Span},
};

use super::{Error, Input};

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Runtime,
    Eof,
    Lex,
    Incomplete(Needed),
    Nom(nom::error::ErrorKind),
    Predicate,
    Grammar,
    Chain(ONode<ErrorKind>, Box<ErrorKind>),
    Convert(Node<ConvertKind>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConvertKind {
    TargetExpr(TargetExpr),
}

impl<'input> ParseError<Input<'input>> for Error {
    fn from_error_kind(input: Input<'input>, kind: nom::error::ErrorKind) -> Self {
        ONode::new(Span::get0(input), ErrorKind::Nom(kind))
    }

    fn append(input: Input<'input>, kind: nom::error::ErrorKind, other: Self) -> Self {
        ONode::new(
            Span::get0(input),
            ErrorKind::Chain(other, Box::new(ErrorKind::Nom(kind))),
        )
    }
}
