use nom::{error::ParseError, Needed};

#[derive(Debug, PartialEq)]
pub struct Error<I> {
    pub input: I,
    pub code: ErrorKind<I>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind<I> {
    Eof,
    Lex,
    Incomplete(Needed),
    Nom(nom::error::ErrorKind),
    Predicate,
    Grammar,
    Chain(Box<ErrorKind<I>>, Box<Error<I>>),
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Error {
            input,
            code: ErrorKind::Nom(kind),
        }
    }

    fn append(input: I, kind: nom::error::ErrorKind, other: Self) -> Self {
        Error {
            input,
            code: ErrorKind::Chain(Box::new(ErrorKind::Nom(kind)), Box::new(other)),
        }
    }
}
