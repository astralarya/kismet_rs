use super::{BasicBlock, Instruction, Value};

pub enum ValueAction {
    Noop,
}

pub type VInstruction = Instruction<ValueAction, Value, Value>;
pub type VBasicBlock = BasicBlock<ValueAction, Value, Value>;
