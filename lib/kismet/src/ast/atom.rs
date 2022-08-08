use std::{fmt, ops::Deref};

use crate::{
    hlir::{Collection, ListItemKind, Primitive, VBasicBlock, VInstruction, Value, ValueAction},
    types::{fmt_float, Float, Integer, Node},
};

use super::{CompIter, DictItem, DictItemComp, Error, Expr, ListItem};

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Block(Vec<Node<Expr>>),
    Paren(Node<Expr>),
    ListDisplay(Vec<Node<ListItem>>),
    ListComprehension {
        val: Node<ListItem>,
        iter: Vec<Node<CompIter>>,
    },
    DictDisplay(Vec<Node<DictItem>>),
    DictComprehension {
        val: Node<DictItemComp>,
        iter: Vec<Node<CompIter>>,
    },
    Tuple(Vec<Node<ListItem>>),
    Generator {
        val: Node<ListItem>,
        iter: Vec<Node<CompIter>>,
    },
    Id(String),
    String(String),
    Float(Float),
    Integer(Integer),
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
            Self::Block(val) => match val.len() {
                1 => write!(f, "{{{};}}", Node::join(val, "; ")),
                _ => write!(f, "{{{}}}", Node::join(val, "; ")),
            },
            Self::Paren(val) => {
                write!(f, "({})", val)
            }
            Self::ListDisplay(val) => write!(f, "[{}]", Node::join(val, ", ")),
            Self::ListComprehension { val, iter } => {
                write!(f, "[{} {}]", val, Node::join(iter, " "))
            }
            Self::DictDisplay(val) => write!(f, "{{{}}}", Node::join(val, ", ")),
            Self::DictComprehension { val, iter } => {
                write!(f, "{{{} {}}}", val, Node::join(iter, " "))
            }
            Self::Tuple(val) => match val.len() {
                1 => write!(f, "({},)", val[0]),
                _ => write!(f, "({})", Node::join(val, ", ")),
            },
            Self::Generator { val, iter } => {
                write!(f, "({} {})", val, Node::join(iter, " "))
            }
            Self::String(val) => write!(f, r#""{}""#, val),
            Self::Float(val) => fmt_float(f, val),
            Self::Integer(val) => write!(f, "{}", val),
            Self::Id(val) => write!(f, "{}", val),
        }
    }
}

impl TryFrom<Atom> for VInstruction {
    type Error = Error;

    fn try_from(val: Atom) -> Result<Self, Self::Error> {
        match val {
            Atom::Block(x) => Ok(VInstruction::Block(VBasicBlock::try_from(x.iter())?)),
            Atom::Paren(x) => VInstruction::try_from(*x.data),
            Atom::ListDisplay(x) => {
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
                let as_value = x
                    .into_iter()
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
                    });
                match as_value {
                    Ok(x) => Ok(VInstruction::Value(Value::Collection(Collection::List(
                        x.into_iter().map(Node::data).collect::<Vec<_>>(),
                    )))),
                    Err(x) => Ok(VInstruction::Action(ValueAction::ListDisplay(x))),
                }
            }
            Atom::ListComprehension { val: _, iter: _ } => todo!(),
            Atom::DictDisplay(_) => todo!(),
            Atom::DictComprehension { val: _, iter: _ } => todo!(),
            Atom::Tuple(_) => todo!(),
            Atom::Generator { val: _, iter: _ } => todo!(),
            Atom::Id(x) => Ok(VInstruction::Variable(Id(x))),
            Atom::String(x) => Ok(VInstruction::Value(Value::Primitive(Primitive::String(x)))),
            Atom::Float(x) => Ok(VInstruction::Value(Value::Primitive(Primitive::Float(x)))),
            Atom::Integer(x) => Ok(VInstruction::Value(Value::Primitive(Primitive::Integer(x)))),
        }
    }
}
