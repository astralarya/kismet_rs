use std::fmt;

use super::{Args, Atom, Expr, Id};
use crate::{
    exec::{Context, Exec1, Value},
    types::Node,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Primary {
    Attribute(Node<Primary>, Node<Id>),
    Subscription(Node<Primary>, Vec<Node<Expr>>),
    Call(Node<Primary>, Node<Args>),
    Atom(Atom),
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

impl Exec1<Context> for Primary {
    type Result = Value;

    fn exec(&self, c: Context) -> (Context, Self::Result) {
        match self {
            Self::Attribute(_, _) => todo!(),
            Self::Subscription(_, _) => todo!(),
            Self::Call(_, _) => todo!(),
            Self::Atom(x) => x.exec(c),
        }
    }
}
