use std::{fmt, ops::Deref};

use crate::{
    hlir::{self, Primitive, VBasicBlock, VInstruction, Value},
    types::{fmt_float, Float, Integer, Node},
};

use super::{CompIter, DictItem, DictItemComp, Expr, ListItem};

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Block(Vec<Node<Expr>>),
    Paren(Node<ListItem>),
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
                1 => write!(f, "{{{};}}", Node::join(&val, "; ")),
                _ => write!(f, "{{{}}}", Node::join(&val, "; ")),
            },
            Self::Paren(val) => {
                write!(f, "({})", val)
            }
            Self::ListDisplay(val) => write!(f, "[{}]", Node::join(&val, ", ")),
            Self::ListComprehension { val, iter } => {
                write!(f, "[{} {}]", val, Node::join(&iter, " "))
            }
            Self::DictDisplay(val) => write!(f, "{{{}}}", Node::join(&val, ", ")),
            Self::DictComprehension { val, iter } => {
                write!(f, "{{{} {}}}", val, Node::join(&iter, " "))
            }
            Self::Tuple(val) => match val.len() {
                1 => write!(f, "({},)", val[0]),
                _ => write!(f, "({})", Node::join(&val, ", ")),
            },
            Self::Generator { val, iter } => {
                write!(f, "({} {})", val, Node::join(&iter, " "))
            }
            Self::String(val) => write!(f, r#""{}""#, val),
            Self::Float(val) => fmt_float(f, val),
            Self::Integer(val) => write!(f, "{}", val),
            Self::Id(val) => write!(f, "{}", val),
        }
    }
}

impl TryFrom<Atom> for VInstruction {
    type Error = hlir::Error;

    fn try_from(val: Atom) -> Result<Self, Self::Error> {
        match val {
            Atom::Block(x) => Ok(VInstruction::Block(VBasicBlock::try_from(x.iter())?)),
            Atom::Paren(_) => todo!(),
            Atom::ListDisplay(_) => todo!(),
            Atom::ListComprehension { val, iter } => todo!(),
            Atom::DictDisplay(_) => todo!(),
            Atom::DictComprehension { val, iter } => todo!(),
            Atom::Tuple(_) => todo!(),
            Atom::Generator { val, iter } => todo!(),
            Atom::Id(_) => todo!(),
            Atom::String(x) => Ok(VInstruction::Value(Value::Primitive(Primitive::String(x)))),
            Atom::Float(x) => Ok(VInstruction::Value(Value::Primitive(Primitive::Float(x)))),
            Atom::Integer(x) => Ok(VInstruction::Value(Value::Primitive(Primitive::Integer(x)))),
        }
    }
}
