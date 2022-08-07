use crate::types::Node;

use super::{BasicBlock, Error, Exec, Instruction, SymbolTable, SymbolTableResult, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum ValueAction {
    ListItemUnpack,
}

pub type VInstruction = Instruction<ValueAction, Value, Value>;
pub type VBasicBlock = BasicBlock<ValueAction, Value, Value>;

impl Exec<Vec<Value>, Value, Error> for ValueAction {
    fn exec(&self, _: Vec<Value>) -> Result<Value, Error> {
        Ok(Value::default())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Action<T, U, V> {
    pub args: Vec<Node<Instruction<T, U, V>>>,
    pub kind: T,
}

impl<T, U, V, E> Exec<SymbolTable<U>, (SymbolTable<U>, V), Error> for Action<T, U, V>
where
    T: Exec<Vec<V>, V, Error>,
    U: TryFrom<V, Error = E> + Clone + Default,
    V: From<U> + Clone + Default,
    Error: From<E>,
{
    fn exec(&self, i: SymbolTable<U>) -> SymbolTableResult<U, V> {
        let (i, args) =
            self.args
                .iter()
                .fold::<Result<_, Error>, _>(Ok((i, vec![])), |acc, val| {
                    let (i, mut vec) = acc?;
                    let (i, val) = val.exec(i)?;
                    vec.push(val);
                    Ok((i, vec))
                })?;
        Ok((i, self.kind.exec(args)?))
    }
}
