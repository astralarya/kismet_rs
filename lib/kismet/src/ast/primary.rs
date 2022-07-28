use std::fmt;

use super::{Atom, Expr};
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Primary {
    Attribute(Node<Primary>, Node<String>),
    Subscription(Node<Primary>, Vec<Node<Expr>>),
    Call(Node<Primary>, Node<Args>),
    Atom(Atom),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Args {
    pub args: Vec<Node<Expr>>,
    pub kwargs: Vec<(String, Node<Expr>)>,
}

impl fmt::Display for Primary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attribute(lhs, rhs) => write!(f, "{}.{}", lhs, rhs),
            Self::Subscription(lhs, rhs) => {
                write!(f, "{}[{}]", lhs, Node::join(rhs, ", "))
            }
            Self::Call(lhs, val) => write!(f, "{}({})", lhs, val),
            Self::Atom(val) => write!(f, "{}", val),
        }
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Node::join(&self.args, ", "))?;
        if self.args.len() > 0 && self.kwargs.len() > 0 {
            write!(f, ", ")?
        }
        let kwargs = self
            .kwargs
            .iter()
            .map(|(key, val)| format!("{}={}", key, val))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", kwargs)
    }
}
