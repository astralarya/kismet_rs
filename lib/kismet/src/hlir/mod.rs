mod actions;
mod collection;
mod error;
mod exec;
mod item;
mod primitive;
mod program;
mod symbol;
mod value;

pub use actions::*;
pub use collection::*;
pub use error::*;
pub use exec::*;
pub use item::*;
pub use primitive::*;
pub use program::*;
pub use symbol::*;
pub use value::*;

use crate::{ast, types::Node};

pub fn compile<T>(input: Node<T>) -> Result<Node<VBasicBlock>, Node<Error>>
where
    VBasicBlock: TryFrom<T, Error = ast::Error>,
{
    Node::<VBasicBlock>::try_convert_from(input).map_err(Node::<Error>::convert_from)
}
