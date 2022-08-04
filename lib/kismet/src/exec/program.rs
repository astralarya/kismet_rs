use crate::types::Node;

use super::{Stack, Value};

pub struct Program(Box<dyn Fn(Stack) -> Value>);

pub enum Instruction {
    Effect(Box<dyn FnOnce(Stack) -> Stack>),
    Return(Box<dyn FnOnce(Stack) -> Value>),
}
