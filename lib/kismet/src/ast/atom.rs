use std::fmt;

use crate::types::{Float, Node, Integer};

use super::{CompIter, Expr, KeyDatum, SpreadItem};

#[derive(Debug, PartialEq)]
pub enum Atom<'input> {
    Parentheses(Node<Expr<'input>>),
    Statements(Node<Expr<'input>>),
    ListDisplay(Vec<Node<SpreadItem<'input>>>),
    ListComprehension {
        val: Node<Expr<'input>>,
        iter: Vec<Node<CompIter<'input>>>,
    },
    DictDisplay(Vec<Node<KeyDatum<'input>>>),
    DictComprehension {
        key: Node<Expr<'input>>,
        val: Node<Expr<'input>>,
        iter: Vec<Node<CompIter<'input>>>,
    },
    Tuple(Vec<Node<Expr<'input>>>),
    Id(&'input str),
    String(String),
    Float(Float),
    Integer(Integer),
}

impl fmt::Display for Atom<'_> {
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
