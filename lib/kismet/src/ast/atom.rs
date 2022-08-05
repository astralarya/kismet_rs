use std::{fmt, ops::Deref};

use crate::types::{Float, Integer, Node};

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
            Self::Float(val) => write!(f, "{}", val),
            Self::Integer(val) => write!(f, "{}", val),
            Self::Id(val) => write!(f, "{}", val),
        }
    }
}

/*
impl Exec1<Context> for Atom {
    type Result = Value;

    fn exec(&self, c: Context) -> (Context, Self::Result) {
        match self {
            Self::Block(_) => todo!(),
            Self::Paren(_) => todo!(),
            Self::ListDisplay(_) => todo!(),
            Self::ListComprehension { val, iter } => todo!(),
            Self::DictDisplay(_) => todo!(),
            Self::DictComprehension { val, iter } => todo!(),
            Self::Tuple(_) => todo!(),
            Self::Generator { val, iter } => todo!(),
            Self::Id(_) => todo!(),
            Self::String(x) => (c, Value::Primitive(Primitive::String(x.clone()))),
            Self::Float(x) => (c, Value::Primitive(Primitive::Float(*x))),
            Self::Integer(x) => (c, Value::Primitive(Primitive::Integer(*x))),
        }
    }
}

 */
