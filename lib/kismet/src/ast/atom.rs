use std::fmt;

use crate::types::{Float, Integer, Node};

use super::{CompIter, Expr, KeyDatum, SpreadItem};

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Parentheses(Node<Expr>),
    Statements(Node<Expr>),
    ListDisplay(Vec<Node<SpreadItem>>),
    ListComprehension {
        val: Node<Expr>,
        iter: Vec<Node<CompIter>>,
    },
    DictDisplay(Vec<Node<KeyDatum>>),
    DictComprehension {
        key: Node<Expr>,
        val: Node<Expr>,
        iter: Vec<Node<CompIter>>,
    },
    Tuple(Vec<Node<Expr>>),
    Id(String),
    String(String),
    Float(Float),
    Integer(Integer),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Atom::Parentheses(val) => {
                write!(f, "({})", val)
            }
            Atom::Statements(val) => {
                write!(f, "{{{}}}", val)
            }
            Atom::ListDisplay(val) => write!(f, "[{}]", Node::vec_to_string(&val, ", ")),
            Atom::ListComprehension { val, iter } => {
                write!(f, "[{} {}]", val, Node::vec_to_string(&iter, " "))
            }
            Atom::DictDisplay(val) => write!(f, "{{{}}}", Node::vec_to_string(&val, ", ")),
            Atom::DictComprehension { key, val, iter } => {
                write!(
                    f,
                    "{{{}: {} {}}}",
                    key,
                    val,
                    Node::vec_to_string(&iter, ", ")
                )
            }
            Atom::Tuple(val) => match val.len() {
                1 => write!(f, "({},)", val[0]),
                _ => write!(f, "({})", Node::vec_to_string(&val, ", ")),
            },
            Atom::String(val) => write!(f, r#""{}""#, val),
            Atom::Float(val) => write!(f, "{}", val),
            Atom::Integer(val) => write!(f, "{}", val),
            Atom::Id(val) => write!(f, "{}", val),
        }
    }
}
