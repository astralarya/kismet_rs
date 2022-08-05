use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::ast::Id;

use super::{Error, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct SymbolIdx {
    map: HashMap<Id, usize>,
    len: usize,
}

impl SymbolIdx {
    pub fn new() -> Self {
        SymbolIdx {
            map: HashMap::new(),
            len: 0,
        }
    }

    pub fn get(&mut self, key: Id) -> usize {
        let len = self.map.len();
        *self.map.entry(key).or_insert(len)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SymbolTable<V>(HashMap<Id, V>);

pub type SymbolTableResult<U, V> = Result<(SymbolTable<U>, V), Error>;

impl<V> SymbolTable<V>
where
    V: Clone + Default,
{
    pub fn new() -> Self {
        SymbolTable(HashMap::new())
    }

    pub fn get(&mut self, key: Id) -> V {
        self.0.entry(key).or_default().clone()
    }

    pub fn set(&mut self, key: Id, val: V) -> Option<V> {
        self.0.insert(key, val)
    }
}

//

#[derive(Clone, Debug, PartialEq)]
pub struct Stack {
    val: Vec<Value>,
    pos: Vec<usize>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            val: vec![],
            pos: vec![],
        }
    }

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
