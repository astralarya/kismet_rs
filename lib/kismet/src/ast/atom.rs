use std::fmt;

use crate::types::{Float, Integer, Node};

use super::{CompIter, DictItem, DictItemComp, Expr, ListItem};

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Stmts(Vec<Node<Expr>>),
    Parenth(Node<Expr>),
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

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Atom::Stmts(val) => match val.len() {
                1 => write!(f, "{{{};}}", Node::vec_to_string(&val, "; ")),
                _ => write!(f, "{{{}}}", Node::vec_to_string(&val, "; ")),
            },
            Atom::Parenth(val) => {
                write!(f, "({})", val)
            }
            Atom::ListDisplay(val) => write!(f, "[{}]", Node::vec_to_string(&val, ", ")),
            Atom::ListComprehension { val, iter } => {
                write!(f, "[{} {}]", val, Node::vec_to_string(&iter, " "))
            }
            Atom::DictDisplay(val) => write!(f, "{{{}}}", Node::vec_to_string(&val, ", ")),
            Atom::DictComprehension { val, iter } => {
                write!(f, "{{{} {}}}", val, Node::vec_to_string(&iter, " "))
            }
            Atom::Tuple(val) => match val.len() {
                1 => write!(f, "({},)", val[0]),
                _ => write!(f, "({})", Node::vec_to_string(&val, ", ")),
            },
            Atom::Generator { val, iter } => {
                write!(f, "({} {})", val, Node::vec_to_string(&iter, " "))
            }
            Atom::String(val) => write!(f, r#""{}""#, val),
            Atom::Float(val) => write!(f, "{}", val),
            Atom::Integer(val) => write!(f, "{}", val),
            Atom::Id(val) => write!(f, "{}", val),
        }
    }
}
