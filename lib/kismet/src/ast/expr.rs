use std::fmt;

use crate::types::Node;

use super::{ArgsDef, Atom, Branch, ExprBlock, Loop, Op, Primary, Target};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Node<Target>, Node<Expr>),
    Function {
        args: Node<ArgsDef>,
        block: Node<ExprBlock>,
    },
    Branch(Branch),
    Loop(Loop),
    Op(Op),
    Primary(Primary),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(lhs, rhs) => write!(f, "{} := {}", lhs, rhs),
            Self::Branch(val) => write!(f, "{}", val),
            Self::Loop(val) => write!(f, "{}", val),
            Self::Function { args, block } => {
                write!(f, "({}) => {}", args, block)
            }
            Self::Op(val) => write!(f, "{}", val),
            Self::Primary(val) => write!(f, "{}", val),
        }
    }
}

impl TryFrom<&Node<Expr>> for Node<String> {
    type Error = ();

    fn try_from(val: &Node<Expr>) -> Result<Self, Self::Error> {
        match &*val.data {
            Expr::Primary(Primary::Atom(Atom::Id(x))) => Ok(Node::new(val.span, x.clone())),
            _ => Err(()),
        }
    }
}
