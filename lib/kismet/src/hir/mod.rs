mod actions;
mod block;
mod collection;
mod error;
mod exec;
mod instruction;
mod item;
mod primitive;
mod symbol;
mod value;

pub use actions::*;
pub use block::*;
pub use collection::*;
pub use error::*;
pub use exec::*;
pub use instruction::*;
pub use item::*;
pub use primitive::*;
pub use symbol::*;
pub use value::*;

use crate::{ast, types::Node};

pub fn compile<T>(input: Node<T>) -> Result<Node<Block>, Node<Error>>
where
    Block: TryFrom<T, Error = ast::Error>,
{
    Node::<Block>::try_convert_from(input).map_err(Node::<Error>::convert_from)
}
