use nom::{error::ParseError, Needed};

use crate::types::{Node, Span};

#[derive(Debug, PartialEq)]
pub enum Error {
    Eof,
    Lex,
    Incomplete(Needed),
    Nom(nom::error::ErrorKind),
    Predicate,
    Grammar,
    Chain(Node<Error>, Box<Error>),
}

impl<T> ParseError<&[Node<T>]> for Node<Error> {
    fn from_error_kind(input: &[Node<T>], kind: nom::error::ErrorKind) -> Self {
        Node::new(Span::from_iter(input), Error::Nom(kind))
    }

    fn append(input: &[Node<T>], kind: nom::error::ErrorKind, other: Self) -> Self {
        Node::new(
            Span::from_iter(input),
            Error::Chain(other, Box::new(Error::Nom(kind))),
        )
    }
}
