use std::slice::Iter;

use crate::{ast, types::Node};

use super::{Error, Exec, Instruction, SymbolTable, SymbolTableResult, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Block(pub Vec<Node<Instruction>>);

impl Exec<SymbolTable, (SymbolTable, Value), Error> for Block {
    fn exec(&self, i: SymbolTable) -> SymbolTableResult {
        self.0
            .iter()
            .fold(Ok((i, Value::default())), move |acc, val| {
                let (i, _) = acc?;
                Ok(val.exec(i)?)
            })
    }
}

impl<N> TryFrom<Iter<'_, Node<N>>> for Block
where
    Instruction: TryFrom<N, Error = ast::Error>,
    N: Clone,
{
    type Error = ast::Error;

    fn try_from(val: Iter<Node<N>>) -> Result<Self, Self::Error> {
        match val
            .map(|x| Node::<Instruction>::try_convert_from(x.clone()))
            .collect::<Result<_, _>>()
        {
            Ok(x) => Ok(Block(x)),
            Err(x) => Err(ast::Error::Node(x)),
        }
    }
}
