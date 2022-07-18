use std::fmt;

use crate::types::{Float, Integer};

use super::{CompIter, Expr, KeyDatum, Node, SpreadItem};

#[derive(Debug, PartialEq)]
pub enum Atom<'input> {
    Expression(Node<Expr<'input>>),
    Statements(Node<Expr<'input>>),
    ListDisplay(Vec<Node<SpreadItem<'input>>>),
    ListComprehension {
        value: Node<Expr<'input>>,
        iter: Vec<Node<CompIter<'input>>>,
    },
    DictDisplay(Vec<Node<KeyDatum<'input>>>),
    DictComprehension {
        key: Node<Expr<'input>>,
        value: Node<Expr<'input>>,
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
            Atom::Expression(node) => {
                write!(f, "({})", node,)
            }
            Atom::Statements(node) => {
                write!(f, "{{{}}}", node,)
            }
            Atom::ListDisplay(nodes) => write!(f, "[{}]", Node::vec_to_string(&nodes, ", ")),
            Atom::ListComprehension { value, iter } => {
                write!(f, "[{} {}]", value, Node::vec_to_string(&iter, " "))
            }
            Atom::DictDisplay(nodes) => write!(f, "{{{}}}", Node::vec_to_string(&nodes, ", ")),
            Atom::DictComprehension { key, value, iter } => {
                write!(
                    f,
                    "{{{}: {} {}}}",
                    key,
                    value,
                    Node::vec_to_string(&iter, ", ")
                )
            }
            Atom::Tuple(nodes) => match nodes.len() {
                1 => write!(f, "({},)", nodes[0]),
                _ => write!(f, "({})", Node::vec_to_string(&nodes, ", ")),
            },
            Atom::String(s) => write!(f, r#""{}""#, s),
            Atom::Float(n) => write!(f, "{}", n),
            Atom::Integer(n) => write!(f, "{}", n),
            Atom::Id(s) => write!(f, "{}", s),
        }
    }
}
