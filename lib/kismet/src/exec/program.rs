use crate::ast::Id;

use super::SymbolTable;

pub trait Exec<T, U> {
    fn exec(&self, i: SymbolTable<T>) -> (SymbolTable<T>, U);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program<T, V>(Vec<Instruction<T, V>>)
where
    T: Exec<V, V>;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction<T, V>
where
    T: Exec<V, V>,
{
    Value(V),
    Variable(Id),
    Action(T),
    Assign(Id, Box<Instruction<T, V>>),
}

impl<T, V> Exec<V, V> for Instruction<T, V>
where
    T: Exec<V, V>,
    V: Clone + Default,
{
    fn exec(&self, i: SymbolTable<V>) -> (SymbolTable<V>, V) {
        match self {
            Self::Value(x) => (i, x.clone()),
            Self::Variable(k) => {
                let mut i = i;
                let v = i.get(k.clone());
                (i, v)
            }
            Self::Action(x) => {
                /*
                let (i, args) = args.iter().fold((i, vec![]), |(i, mut vec), val| {
                    let (i, val) = val.exec(i);
                    vec.push(val);
                    (i, vec)
                });
                 */
                x.exec(i)
            }
            Self::Assign(key, val) => {
                let (mut i, val) = val.exec(i);
                i.set(key.clone(), val.clone());
                (i, val)
            }
        }
    }
}

impl<T, V> Exec<V, V> for Program<T, V>
where
    T: Exec<V, V>,
    V: Clone + Default,
{
    fn exec(&self, i: SymbolTable<V>) -> (SymbolTable<V>, V) {
        self.0
            .iter()
            .fold((i, V::default()), move |(i, _), val| val.exec(i))
    }
}
