use std::fmt;

use crate::types::Node;

use super::{Expr, Id, Target};

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Return(Node<Expr>),
    Break {
        id: Option<Node<Id>>,
        val: Node<Expr>,
    },
    Assign {
        tar: Node<Target>,
        val: Node<Expr>,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Return(x) => write!(f, "return {}", x),
            Self::Break { id, val } => match id {
                Some(id) => write!(f, "break :{}: {}", id, val),
                None => write!(f, "break {}", val),
            },
            Self::Assign { tar, val } => write!(f, "{} = {}", tar, val),
        }
    }
}
