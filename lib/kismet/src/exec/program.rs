use crate::ast::Id;

use super::SymbolTable;

#[derive(Clone, Debug, PartialEq)]
pub struct Program<T, V>(Vec<Instruction<T, V>>)
where
    T: Action<V>;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction<T, V>
where
    T: Action<V>,
{
    Value(V),
    Variable(Id),
    Action {
        args: Vec<Instruction<T, V>>,
        action: T,
    },
    Assign(Id, Box<Instruction<T, V>>),
}

pub trait Exec<V> {
    fn exec(&self, i: SymbolTable<V>) -> (SymbolTable<V>, V);
}

pub trait Action<V> {
    fn action(&self, i: Vec<V>) -> V;
}

impl<T, V> Exec<V> for Instruction<T, V>
where
    T: Action<V>,
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
            Self::Action { args, action } => {
                let (i, args) = args.iter().fold((i, vec![]), |(i, mut vec), val| {
                    let (i, val) = val.exec(i);
                    vec.push(val);
                    (i, vec)
                });
                (i, action.action(args))
            }
            Self::Assign(key, val) => {
                let (mut i, val) = val.exec(i);
                i.set(key.clone(), val.clone());
                (i, val)
            }
        }
    }
}

impl<T, V> Exec<V> for Program<T, V>
where
    T: Action<V>,
    V: Clone + Default,
{
    fn exec(&self, i: SymbolTable<V>) -> (SymbolTable<V>, V) {
        self.0
            .iter()
            .fold((i, V::default()), move |(i, _), val| val.exec(i))
    }
}
