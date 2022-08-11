use std::slice::Iter;

use crate::{
    ast::{self, Id},
    types::Node,
};

use super::{Error, Exec, SymbolTable, SymbolTableResult};

#[derive(Clone, Debug, PartialEq)]
pub struct Block<T, U, V>(pub Vec<Node<Instruction<T, U, V>>>);

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction<T, U, V> {
    Value(V),
    Variable(Id),
    Action(T),
    Block(Block<T, U, V>),
    Assign(Id, Node<Instruction<T, U, V>>),
    Symbol(U),
}

impl<T, U, V, E> Exec<SymbolTable<U>, (SymbolTable<U>, V), Error> for Instruction<T, U, V>
where
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V), Error>,
    U: TryFrom<V, Error = E> + Clone + Default,
    V: From<U> + Clone + Default,
    Error: From<E>,
{
    fn exec(&self, i: SymbolTable<U>) -> SymbolTableResult<U, V> {
        match self {
            Self::Value(x) => Ok((i, x.clone())),
            Self::Variable(key) => {
                let mut i = i;
                let val = i.get(key.clone());
                Ok((i, V::from(val)))
            }
            Self::Action(x) => x.exec(i),
            Self::Block(x) => x.exec(i),
            Self::Assign(key, val) => {
                let (mut i, val) = val.exec(i)?;
                i.set(key.clone(), U::try_from(val.clone())?);
                Ok((i, val))
            }
            Self::Symbol(x) => Ok((i, V::from(x.clone()))),
        }
    }
}

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
