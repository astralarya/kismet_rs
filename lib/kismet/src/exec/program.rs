use crate::ast::Id;

use super::{Exec, SymbolTable};

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

impl<F, T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V)> for Call<F, T, U, V>
where
    F: Exec<Vec<V>, V>,
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V)>,
    V: From<U> + Clone + Default,
    U: From<V> + Clone + Default,
{
    fn exec(&self, i: SymbolTable<U>) -> (SymbolTable<U>, V) {
        let (i, args) = self.args.iter().fold((i, vec![]), |(i, mut vec), val| {
            let (i, val) = val.exec(i);
            vec.push(val);
            (i, vec)
        });
        (i, self.action.exec(args))
    }
}

impl<T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V)> for Instruction<T, U, V>
where
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V)>,
    V: From<U> + Clone,
    U: From<V> + Clone + Default,
{
    fn exec(&self, i: SymbolTable<U>) -> (SymbolTable<U>, V) {
        match self {
            Self::Value(x) => (i, x.clone()),
            Self::Variable(key) => {
                let mut i = i;
                let val = i.get(key.clone());
                (i, V::from(val))
            }
            Self::Action(x) => x.exec(i),
            Self::Assign(key, val) => {
                let (mut i, val) = val.exec(i);
                i.set(key.clone(), U::from(val.clone()));
                (i, val)
            }
            Self::Symbol(x) => (i, V::from(x.clone())),
        }
    }
}

impl<T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V)> for Program<T, U, V>
where
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V)>,
    V: From<U> + Clone + Default,
    U: From<V> + Clone + Default,
{
    fn exec(&self, i: SymbolTable<U>) -> (SymbolTable<U>, V) {
        self.0
            .iter()
            .fold((i, V::default()), move |(i, _), val| val.exec(i))
    }
}
