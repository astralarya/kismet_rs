use std::slice::Iter;

use crate::{ast, types::Node};

use super::{Error, Exec, Instruction, SymbolTable, SymbolTableResult};

#[derive(Clone, Debug, PartialEq)]
pub struct Block<T, U, V>(pub Vec<Node<Instruction<T, U, V>>>);

impl<T, U, V, E> Exec<SymbolTable<U>, (SymbolTable<U>, V), Error> for Block<T, U, V>
where
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V), Error>,
    U: TryFrom<V, Error = E> + Clone + Default,
    V: From<U> + Clone + Default,
    Error: From<E>,
{
    fn exec(&self, i: SymbolTable<U>) -> SymbolTableResult<U, V> {
        self.0.iter().fold(Ok((i, V::default())), move |acc, val| {
            let (i, _) = acc?;
            Ok(val.exec(i)?)
        })
    }
}

impl<N, T, U, V> TryFrom<Iter<'_, Node<N>>> for Block<T, U, V>
where
    Instruction<T, U, V>: TryFrom<N, Error = ast::Error>,
    N: Clone,
{
    type Error = ast::Error;

    fn try_from(val: Iter<Node<N>>) -> Result<Self, Self::Error> {
        match val
            .map(|x| Node::<Instruction<T, U, V>>::try_convert_from(x.clone()))
            .collect::<Result<_, _>>()
        {
            Ok(x) => Ok(Block(x)),
            Err(x) => Err(ast::Error::Node(x)),
        }
    }
}
