use indexmap::IndexMap;

use crate::{ast::Id, hir::Primitive, types::Node};

use super::{
    Collection, DictItem, Error, Exec, Instruction, ListItem, SymbolTable, SymbolTableResult, Value,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Tuple(Vec<Node<ListItem<Instruction>>>),
    ListDisplay(Vec<Node<ListItem<Instruction>>>),
    DictDisplay(Vec<Node<DictItem<Instruction>>>),
}

impl Exec<SymbolTable, (SymbolTable, Value), Error> for Action {
    fn exec(&self, i: SymbolTable) -> SymbolTableResult {
        fn iter_list(
            i: SymbolTable,
            x: &[Node<ListItem<Instruction>>],
        ) -> Result<(SymbolTable, Vec<Value>), Error> {
            x.iter()
                .fold::<Result<_, Error>, _>(Ok((i, vec![])), |acc, val| {
                    let (i, mut vec) = acc?;
                    let (val, spread) = match &*val.data {
                        ListItem::Expr(x) => (x, false),
                        ListItem::Spread(x) => (x, true),
                    };
                    let (i, val) = val.exec(i)?;
                    match (spread, val) {
                        (false, val) => vec.push(val),
                        (true, Value::Collection(Collection::List(mut val))) => {
                            vec.append(&mut val)
                        }
                        (true, Value::Collection(Collection::Tuple(mut val))) => {
                            vec.append(&mut val)
                        }
                        (true, _) => return Err(Error::TypeMismatch),
                    }
                    Ok((i, vec))
                })
        }

        match self {
            Action::Tuple(x) => {
                let (i, val) = iter_list(i, x)?;
                Ok((i, Value::Collection(Collection::Tuple(val))))
            }
            Action::ListDisplay(x) => {
                let (i, val) = iter_list(i, x)?;
                Ok((i, Value::Collection(Collection::List(val))))
            }
            Action::DictDisplay(x) => x
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
pub struct Args(pub Vec<Node<Instruction>>);

impl Exec<SymbolTable, (SymbolTable, Vec<Value>), Error> for Args {
    fn exec(&self, i: SymbolTable) -> Result<(SymbolTable, Vec<Value>), Error> {
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
