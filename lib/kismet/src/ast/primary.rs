use std::fmt;

use super::{Args, Atom, Error, Expr, Id};
use crate::{hir::Instruction, types::Node};

#[derive(Clone, Debug, PartialEq)]
pub enum Primary {
    Attribute(Node<Primary>, Node<Id>),
    Index(Node<Primary>, Node<usize>),
    Subscription(Node<Primary>, Vec<Node<Expr>>),
    Call(Node<Primary>, Node<Args>),
    Atom(Atom),
}

impl fmt::Display for Primary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attribute(lhs, rhs) => write!(f, "{}.{}", lhs, rhs),
            Self::Index(lhs, rhs) => write!(f, "{}.{}", lhs, rhs),
            Self::Subscription(lhs, rhs) => {
                write!(f, "{}[{}]", lhs, Node::join(rhs, ", "))
            }
            Self::Call(lhs, val) => write!(f, "{}({})", lhs, val),
            Self::Atom(val) => write!(f, "{}", val),
        }
    }
}

impl TryFrom<Primary> for Instruction {
    type Error = Error;

    fn try_from(val: Primary) -> Result<Self, Self::Error> {
        match val {
            Primary::Attribute(_, _) => todo!(),
            Primary::Index(_, _) => todo!(),
            Primary::Subscription(_, _) => todo!(),
            Primary::Call(_, _) => todo!(),
            Primary::Atom(x) => Instruction::try_from(x),
        }
    }
}
