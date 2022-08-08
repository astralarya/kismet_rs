use std::convert::Infallible;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Never,
    TypeMismatch,
    InvalidOp,
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Self::Never
    }
}
