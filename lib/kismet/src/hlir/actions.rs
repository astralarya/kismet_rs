use syn::Block;

use super::{BasicBlock, Error, Exec, Instruction, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum ValueAction {
    Noop,
}

pub type VInstruction = Instruction<ValueAction, Value, Value>;
pub type VBasicBlock = BasicBlock<ValueAction, Value, Value>;

impl Exec<Vec<Value>, Value, Error> for ValueAction {
    fn exec(&self, i: Vec<Value>) -> Result<Value, Error> {
        Ok(Value::default())
    }
}
