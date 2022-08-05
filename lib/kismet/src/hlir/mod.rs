mod actions;
mod error;
mod exec;
mod primitive;
mod program;
mod symbol;
mod types;

pub use actions::*;
pub use error::*;
pub use exec::*;
pub use primitive::*;
pub use program::*;
pub use symbol::*;
pub use types::*;

use crate::types::Node;

pub fn exec<'a, T, E>(input: Node<T>) -> Result<Value, Error>
where
    VBasicBlock: TryFrom<T, Error = E>,
    Error: From<E>,
{
    let program = Node::<VBasicBlock>::try_convert_from(input)?;
    let (_, val) = program.exec(SymbolTable::<Value>::new())?;
    Ok(val)
}
