use nom::{error::ParseError, Needed};

use crate::types::{ONode, Span};

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
}

impl<'input> ParseError<Input<'input>> for Error {
    fn from_error_kind(input: Input<'input>, kind: nom::error::ErrorKind) -> Self {
        ONode::new(
            input.get(0).map(|x| Span::from(x.span)),
            ErrorKind::Nom(kind),
        )
    }

    fn append(input: Input<'input>, kind: nom::error::ErrorKind, other: Self) -> Self {
        ONode::new(
            input.get(0).map(|x| Span::from(x.span)),
            ErrorKind::Chain(other, Box::new(ErrorKind::Nom(kind))),
        )
    }
}
