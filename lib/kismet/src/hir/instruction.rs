use crate::{ast::Id, types::Node};

use super::{Block, Error, Exec, SymbolTable, SymbolTableResult};

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
