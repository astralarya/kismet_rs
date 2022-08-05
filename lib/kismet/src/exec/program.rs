use crate::ast::Id;

use super::{Error, Exec, SymbolTable};

#[derive(Clone, Debug, PartialEq)]
pub struct Program<T, U, V>(Vec<Instruction<T, U, V>>);

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction<T, U, V> {
    Value(V),
    Variable(Id),
    Action(T),
    Assign(Id, Box<Instruction<T, U, V>>),
    Symbol(U),
}

pub struct Call<F, T, U, V> {
    pub args: Vec<Instruction<T, U, V>>,
    pub action: F,
}

pub type SymbolTableResult<U, V> = Result<(SymbolTable<U>, V), Error>;

impl<F, T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V), Error> for Call<F, T, U, V>
where
    F: Exec<Vec<V>, V, Error>,
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V), Error>,
    V: From<U> + Clone + Default,
    U: From<V> + Clone + Default,
{
    fn exec(&self, i: SymbolTable<U>) -> SymbolTableResult<U, V> {
        let (i, args) = self.args.iter().fold(Ok((i, vec![])), |acc, val| {
            let (i, mut vec) = acc?;
            let (i, val) = val.exec(i)?;
            vec.push(val);
            Ok((i, vec))
        })?;
        Ok((i, self.action.exec(args)?))
    }
}

impl<T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V), Error> for Instruction<T, U, V>
where
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V), Error>,
    V: From<U> + Clone,
    U: From<V> + Clone + Default,
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
            Self::Assign(key, val) => {
                let (mut i, val) = val.exec(i)?;
                i.set(key.clone(), U::from(val.clone()));
                Ok((i, val))
            }
            Self::Symbol(x) => Ok((i, V::from(x.clone()))),
        }
    }
}

impl<T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V), Error> for Program<T, U, V>
where
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V), Error>,
    V: From<U> + Clone + Default,
    U: From<V> + Clone + Default,
{
    fn exec(&self, i: SymbolTable<U>) -> SymbolTableResult<U, V> {
        self.0.iter().fold(Ok((i, V::default())), move |acc, val| {
            let (i, _) = acc?;
            val.exec(i)
        })
    }
}
