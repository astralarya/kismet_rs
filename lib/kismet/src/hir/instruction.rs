use crate::{ast::Id, types::Node};

use super::{Action, Block, Error, Exec, SymbolTable, SymbolTableResult, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Value(Value),
    Variable(Id),
    Action(Action),
    Block(Block),
    Assign(Id, Node<Instruction>),
    Symbol(Value),
}

impl Exec<SymbolTable, (SymbolTable, Value), Error> for Instruction {
    fn exec(&self, i: SymbolTable) -> SymbolTableResult {
        match self {
            Self::Value(x) => Ok((i, x.clone())),
            Self::Variable(key) => {
                let mut i = i;
                let val = i.get(key.clone());
                Ok((i, val))
            }
            Self::Action(x) => x.exec(i),
            Self::Block(x) => x.exec(i),
            Self::Assign(key, val) => {
                let (mut i, val) = val.exec(i)?;
                i.set(key.clone(), val.clone());
                Ok((i, val))
            }
            Self::Symbol(x) => Ok((i, x.clone())),
        }
    }
}
