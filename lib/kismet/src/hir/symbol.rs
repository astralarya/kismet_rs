use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::ast::Id;

use super::{Error, Value};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SymbolIdx {
    map: HashMap<Id, usize>,
    len: usize,
}

impl SymbolIdx {
    pub fn get(&mut self, key: Id) -> usize {
        let len = self.map.len();
        *self.map.entry(key).or_insert(len)
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SymbolTable(HashMap<Id, Value>);

pub type SymbolTableResult = Result<(SymbolTable, Value), Error>;

impl SymbolTable {
    pub fn get(&mut self, key: Id) -> Value {
        self.0.entry(key).or_default().clone()
    }

    pub fn set(&mut self, key: Id, val: Value) -> Option<Value> {
        self.0.insert(key, val)
    }
}

//

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Stack {
    val: Vec<Value>,
    pos: Vec<usize>,
}

impl Stack {
    pub fn pos(&self) -> usize {
        *self.pos.last().unwrap_or(&0)
    }

    pub fn push_frame(&mut self) {
        self.pos.push(self.val.len())
    }

    pub fn pop_frame(&mut self) -> Option<usize> {
        self.pos.pop()
    }
}

impl Deref for Stack {
    type Target = [Value];

    fn deref(&self) -> &Self::Target {
        &self.val[self.pos()..]
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let pos = self.pos();
        &mut self.val[pos..]
    }
}
