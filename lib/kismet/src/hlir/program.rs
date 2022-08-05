use crate::{ast::Id, types::Node};

use super::{Error, Exec, SymbolTable, SymbolTableResult};

#[derive(Clone, Debug, PartialEq)]
pub struct BasicBlock<T, U, V>(pub Vec<Node<Instruction<T, U, V>>>);

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction<T, U, V> {
    Value(V),
    Variable(Id),
    Action(Action<T, U, V>),
    Assign(Id, Box<Node<Instruction<T, U, V>>>),
    Symbol(U),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Action<T, U, V> {
    pub args: Vec<Node<Instruction<T, U, V>>>,
    pub kind: T,
}

impl<T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V), Error> for Action<T, U, V>
where
    T: Exec<Vec<V>, V, Error>,
    V: TryFrom<U, Error = Error> + Clone,
    U: TryFrom<V, Error = Error> + Clone + Default,
{
    fn exec(&self, i: SymbolTable<U>) -> SymbolTableResult<U, V> {
        let (i, args) = self.args.iter().fold(Ok((i, vec![])), |acc, val| {
            let (i, mut vec) = acc?;
            let (i, val) = val.exec(i)?;
            vec.push(val);
            Ok((i, vec))
        })?;
        Ok((i, self.kind.exec(args)?))
    }
}

impl<T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V), Error> for Instruction<T, U, V>
where
    T: Exec<Vec<V>, V, Error>,
    V: TryFrom<U, Error = Error> + Clone,
    U: TryFrom<V, Error = Error> + Clone + Default,
{
    fn exec(&self, i: SymbolTable<U>) -> SymbolTableResult<U, V> {
        match self {
            Self::Value(x) => Ok((i, x.clone())),
            Self::Variable(key) => {
                let mut i = i;
                let val = i.get(key.clone());
                Ok((i, V::try_from(val)?))
            }
            Self::Action(x) => x.exec(i),
            Self::Assign(key, val) => {
                let (mut i, val) = val.exec(i)?;
                i.set(key.clone(), U::try_from(val.clone())?);
                Ok((i, val))
            }
            Self::Symbol(x) => Ok((i, V::try_from(x.clone())?)),
        }
    }
}

impl<T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V), Error> for BasicBlock<T, U, V>
where
    T: Exec<Vec<V>, V, Error>,
    V: TryFrom<U, Error = Error> + Clone + Default,
    U: TryFrom<V, Error = Error> + Clone + Default,
{
    fn exec(&self, i: SymbolTable<U>) -> SymbolTableResult<U, V> {
        self.0.iter().fold(Ok((i, V::default())), move |acc, val| {
            let (i, _) = acc?;
            val.exec(i)
        })
    }
}
