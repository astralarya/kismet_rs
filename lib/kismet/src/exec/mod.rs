mod program;
mod symbol;
mod types;

pub use program::*;
pub use symbol::*;
pub use types::*;

use crate::types::BaseNode;

#[derive(Clone, Debug, PartialEq)]
pub struct Context {}

impl From<Option<Context>> for Context {
    fn from(val: Option<Context>) -> Self {
        match val {
            Some(val) => val,
            None => Context {},
        }
    }
}

pub trait Exec1<T> {
    type Result;

    fn exec(&self, c: T) -> (T, Self::Result);

    fn exec_from<C>(&self, c: C) -> (T, Self::Result)
    where
        T: From<C>,
    {
        self.exec(T::from(c))
    }
}

impl<T, S, N, R> Exec1<T> for BaseNode<S, N>
where
    N: Exec1<T, Result = R>,
{
    type Result = R;

    fn exec(&self, c: T) -> (T, Self::Result) {
        self.data.exec(c)
    }
}
