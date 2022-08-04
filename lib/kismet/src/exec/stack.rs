use std::ops::{Deref, DerefMut};

use super::Value;

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
