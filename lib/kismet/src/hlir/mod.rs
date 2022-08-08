mod actions;
mod collection;
mod error;
mod exec;
mod primitive;
mod program;
mod symbol;
mod value;

pub use actions::*;
pub use collection::*;
pub use error::*;
pub use exec::*;
pub use primitive::*;
pub use program::*;
pub use symbol::*;
pub use value::*;

use crate::{ast, types::Node};

pub fn exec<T>(input: Node<T>) -> Result<Value, Error>
where
    VBasicBlock: TryFrom<T, Error = ast::Error>,
{
    let program = Node::<VBasicBlock>::try_convert_from(input)?;
    let (_, val) = program.exec(SymbolTable::<Value>::default())?;
    Ok(val)
}
