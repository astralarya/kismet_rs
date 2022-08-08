use crate::types::Node;

use super::{
    BasicBlock, Collection, Error, Exec, Instruction, SymbolTable, SymbolTableResult, Value,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ValueAction {
    ListDisplay(Vec<(ListItemKind, Node<VInstruction>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListItemKind {
    Expr,
    Spread,
}

pub type VInstruction = Instruction<ValueAction, Value, Value>;
pub type VBasicBlock = BasicBlock<ValueAction, Value, Value>;

impl Exec<SymbolTable<Value>, (SymbolTable<Value>, Value), Error> for ValueAction {
    fn exec(&self, i: SymbolTable<Value>) -> SymbolTableResult<Value, Value> {
        match self {
            ValueAction::ListDisplay(x) => {
                let (i, val) =
                    x.iter()
                        .fold::<Result<_, Error>, _>(Ok((i, vec![])), |acc, (kind, val)| {
                            let (i, mut vec) = acc?;
                            let (i, val) = val.exec(i)?;
                            match (kind, val) {
                                (ListItemKind::Expr, val) => vec.push(val),
                                (
                                    ListItemKind::Spread,
                                    Value::Collection(Collection::List(mut val)),
                                ) => vec.append(&mut val),
                                (
                                    ListItemKind::Spread,
                                    Value::Collection(Collection::Tuple(mut val)),
                                ) => vec.append(&mut val),
                                (ListItemKind::Spread, _) => return Err(Error::TypeMismatch),
                            }
                            Ok((i, vec))
                        })?;
                Ok((i, Value::Collection(Collection::List(val))))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Args<T, U, V>(pub Vec<Node<Instruction<T, U, V>>>);

impl<T, U, V, E> Exec<SymbolTable<U>, (SymbolTable<U>, Vec<V>), Error> for Args<T, U, V>
where
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V), Error>,
    U: TryFrom<V, Error = E> + Clone + Default,
    V: From<U> + Clone + Default,
    Error: From<E>,
{
    fn exec(&self, i: SymbolTable<U>) -> SymbolTableResult<U, Vec<V>> {
        self.0
            .iter()
            .fold::<Result<_, Error>, _>(Ok((i, vec![])), |acc, val| {
                let (i, mut vec) = acc?;
                let (i, val) = val.exec(i)?;
                vec.push(val);
                Ok((i, vec))
            })
    }
}
