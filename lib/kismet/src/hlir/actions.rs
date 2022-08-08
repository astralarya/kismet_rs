use indexmap::IndexMap;

use crate::{ast::Id, hlir::Primitive, types::Node};

use super::{
    Block, Collection, DictItem, Error, Exec, Instruction, SymbolTable, SymbolTableResult, Value,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ValueAction {
    Tuple(Vec<Node<(ListItemKind, VInstruction)>>),
    ListDisplay(Vec<Node<(ListItemKind, VInstruction)>>),
    DictDisplay(Vec<Node<VDictItem>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListItemKind {
    Expr,
    Spread,
}

pub type VDictItem = DictItem<VInstruction>;

pub type VInstruction = Instruction<ValueAction, Value, Value>;
pub type VBasicBlock = Block<ValueAction, Value, Value>;
pub type VListItem = (ListItemKind, VInstruction);

impl Exec<SymbolTable<Value>, (SymbolTable<Value>, Value), Error> for ValueAction {
    fn exec(&self, i: SymbolTable<Value>) -> SymbolTableResult<Value, Value> {
        fn iter_list(
            i: SymbolTable<Value>,
            x: &[Node<(ListItemKind, VInstruction)>],
        ) -> Result<(SymbolTable<Value>, Vec<Value>), Error> {
            x.iter()
                .fold::<Result<_, Error>, _>(Ok((i, vec![])), |acc, val| {
                    let (kind, val) = &*val.data;
                    let (i, mut vec) = acc?;
                    let (i, val) = val.exec(i)?;
                    match (kind, val) {
                        (ListItemKind::Expr, val) => vec.push(val),
                        (ListItemKind::Spread, Value::Collection(Collection::List(mut val))) => {
                            vec.append(&mut val)
                        }
                        (ListItemKind::Spread, Value::Collection(Collection::Tuple(mut val))) => {
                            vec.append(&mut val)
                        }
                        (ListItemKind::Spread, _) => return Err(Error::TypeMismatch),
                    }
                    Ok((i, vec))
                })
        }

        match self {
            ValueAction::Tuple(x) => {
                let (i, val) = iter_list(i, x)?;
                Ok((i, Value::Collection(Collection::Tuple(val))))
            }
            ValueAction::ListDisplay(x) => {
                let (i, val) = iter_list(i, x)?;
                Ok((i, Value::Collection(Collection::List(val))))
            }
            ValueAction::DictDisplay(x) => x
                .iter()
                .fold::<Result<(_, IndexMap<_, _>), Error>, _>(
                    Ok((i, IndexMap::new())),
                    |acc, val| match acc {
                        Ok((mut i, mut acc)) => match &*val.data {
                            DictItem::KeyVal { key, val } => {
                                let (i, val) = val.exec(i)?;
                                acc.insert((*key.data).clone(), val);
                                Ok((i, acc))
                            }
                            DictItem::DynKeyVal { key, val } => {
                                let (i, key) = key.exec(i)?;
                                if let Value::Primitive(Primitive::String(key)) = key {
                                    let (i, val) = val.exec(i)?;
                                    acc.insert(Id(key), val);
                                    Ok((i, acc))
                                } else {
                                    Err(Error::TypeMismatch)
                                }
                            }
                            DictItem::Shorthand(x) => {
                                let key = x.clone();
                                let val = i.get(x.clone());
                                acc.insert(key, val);
                                Ok((i, acc))
                            }
                            DictItem::Spread(x) => {
                                let (i, val) = x.exec(i)?;
                                if let Value::Collection(Collection::Dict(val)) = val {
                                    Ok((
                                        i,
                                        val.into_iter().fold(acc, |mut acc, (id, val)| {
                                            acc.insert(id, val);
                                            acc
                                        }),
                                    ))
                                } else {
                                    Err(Error::TypeMismatch)
                                }
                            }
                        },
                        Err(x) => Err(x),
                    },
                )
                .map(|(i, val)| (i, Value::Collection(Collection::Dict(val)))),
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
