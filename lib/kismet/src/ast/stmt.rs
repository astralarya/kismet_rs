use std::fmt;

use crate::types::Node;

use super::{Expr, Id, Target};

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Assign {
        tar: Node<Target>,
        val: Node<Expr>,
    },
    Break {
        id: Option<Node<Id>>,
        val: Node<Expr>,
    },
    Return(Node<Expr>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign { tar, val } => write!(f, "{} = {}", tar, val),
            Self::Break { id, val } => match id {
                Some(id) => write!(f, "break :{}: {}", id, val),
                None => write!(f, "break {}", val),
            },
            Self::Return(x) => write!(f, "return {}", x),
        }
    }
}
