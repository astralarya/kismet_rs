use std::{fmt, ops::Deref};

use crate::{
    hlir::{
        Collection, ListItemKind, Primitive, VBasicBlock, VInstruction, VListItem, Value,
        ValueAction,
    },
    types::{fmt_float, Float, Integer, Node},
};

use super::{CompIter, DictItem, DictItemComp, Error, Expr, ListItem};

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Id(String),
    Integer(Integer),
    Float(Float),
    String(String),
    Paren(Node<Expr>),
    Tuple(Vec<Node<ListItem>>),
    ListDisplay(Vec<Node<ListItem>>),
    DictDisplay(Vec<Node<DictItem>>),
    Generator {
        val: Node<ListItem>,
        iter: Vec<Node<CompIter>>,
    },
    ListComprehension {
        val: Node<ListItem>,
        iter: Vec<Node<CompIter>>,
    },
    DictComprehension {
        val: Node<DictItemComp>,
        iter: Vec<Node<CompIter>>,
    },
    Block(Vec<Node<Expr>>),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id(pub String);

impl Deref for Id {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Id(val) => write!(f, "{}", val),
            Self::Integer(val) => write!(f, "{}", val),
            Self::Float(val) => fmt_float(f, val),
            Self::String(val) => write!(f, r#""{}""#, val),
            Self::Paren(val) => {
                write!(f, "({})", val)
            }
            Self::Tuple(val) => match val.len() {
                1 => write!(f, "({},)", val[0]),
                _ => write!(f, "({})", Node::join(val, ", ")),
            },
            Self::ListDisplay(val) => write!(f, "[{}]", Node::join(val, ", ")),
            Self::DictDisplay(val) => write!(f, "{{{}}}", Node::join(val, ", ")),
            Self::Generator { val, iter } => {
                write!(f, "({} {})", val, Node::join(iter, " "))
            }
            Self::ListComprehension { val, iter } => {
                write!(f, "[{} {}]", val, Node::join(iter, " "))
            }
            Self::DictComprehension { val, iter } => {
                write!(f, "{{{} {}}}", val, Node::join(iter, " "))
            }
            Self::Block(val) => match val.len() {
                1 => write!(f, "{{{};}}", Node::join(val, "; ")),
                _ => write!(f, "{{{}}}", Node::join(val, "; ")),
            },
        }
    }
}

impl TryFrom<Atom> for VInstruction {
    type Error = Error;

    fn try_from(val: Atom) -> Result<Self, Self::Error> {
        fn list_value(
            x: Vec<Node<ListItem>>,
        ) -> Result<Result<Vec<Value>, Vec<Node<VListItem>>>, Error> {
            let (x, err) = x
                .into_iter()
                .map(Node::<(ListItemKind, VInstruction)>::try_from)
                .fold::<(Vec<_>, Vec<_>), _>((vec![], vec![]), |mut acc, val| match val {
                    Ok(x) => {
                        acc.0.push(x);
                        acc
                    }
                    Err(x) => {
                        acc.1.push(x);
                        acc
                    }
                });
            if !err.is_empty() {
                return Err(Error::Vec(err));
            }
            Ok(x.into_iter()
                .map(|x| {
                    Node::convert(
                        |x| match x {
                            (ListItemKind::Expr, crate::hlir::Instruction::Value(x)) => Ok(x),
                            x => Err(x),
                        },
                        x,
                    )
                })
                .fold::<Result<Vec<_>, Vec<_>>, _>(Ok(vec![]), |acc, val| {
                    let span = val.span;
                    match (acc, *val.data) {
                        (Ok(mut acc), Ok(val)) => {
                            acc.push(Node::new(span, val));
                            Ok(acc)
                        }
                        (Ok(acc), Err(val)) => {
                            let mut acc = acc
                                .into_iter()
                                .map(|x| {
                                    Node::convert(
                                        |x| (ListItemKind::Expr, VInstruction::Value(x)),
                                        x,
                                    )
                                })
                                .collect::<Vec<_>>();
                            acc.push(Node::new(span, val));
                            Err(acc)
                        }
                        (Err(mut acc), Ok(val)) => {
                            acc.push(Node::new(
                                span,
                                (ListItemKind::Expr, VInstruction::Value(val)),
                            ));
                            Err(acc)
                        }
                        (Err(mut acc), Err(val)) => {
                            acc.push(Node::new(span, val));
                            Err(acc)
                        }
                    }
                })
                .map(|x| x.into_iter().map(Node::data).collect::<Vec<_>>()))
        }

        match val {
            Atom::Id(x) => Ok(VInstruction::Variable(Id(x))),
            Atom::Integer(x) => Ok(VInstruction::Value(Value::Primitive(Primitive::Integer(x)))),
            Atom::Float(x) => Ok(VInstruction::Value(Value::Primitive(Primitive::Float(x)))),
            Atom::String(x) => Ok(VInstruction::Value(Value::Primitive(Primitive::String(x)))),
            Atom::Paren(x) => VInstruction::try_from(*x.data),
            Atom::Tuple(x) => match list_value(x)? {
                Ok(x) => Ok(VInstruction::Value(Value::Collection(Collection::Tuple(x)))),
                Err(x) => Ok(VInstruction::Action(ValueAction::Tuple(x))),
            },
            Atom::ListDisplay(x) => match list_value(x)? {
                Ok(x) => Ok(VInstruction::Value(Value::Collection(Collection::List(x)))),
                Err(x) => Ok(VInstruction::Action(ValueAction::ListDisplay(x))),
            },
            Atom::DictDisplay(_) => todo!(),
            Atom::Generator { val: _, iter: _ } => todo!(),
            Atom::ListComprehension { val: _, iter: _ } => todo!(),
            Atom::DictComprehension { val: _, iter: _ } => todo!(),
            Atom::Block(x) => Ok(VInstruction::Block(VBasicBlock::try_from(x.iter())?)),
        }
    }
}
