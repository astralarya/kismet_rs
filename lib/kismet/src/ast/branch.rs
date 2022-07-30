use std::fmt;

use crate::types::Node;

use super::{Expr, ExprBlock, ExprEnclosure, Target};

#[derive(Clone, Debug, PartialEq)]
pub enum Branch {
    If {
        val: Node<Expr>,
        t_block: Node<ExprEnclosure>,
        f_block: Node<ExprEnclosure>,
    },
    Match {
        val: Node<Expr>,
        arms: Vec<Node<MatchArm>>,
    },
    For {
        label: Label,
        tar: Node<Target>,
        val: Node<Expr>,
        block: Node<ExprEnclosure>,
    },
    While {
        label: Label,
        val: Node<Expr>,
        block: Node<ExprEnclosure>,
    },
    Loop {
        label: Label,
        block: Node<ExprEnclosure>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchArm {
    tar: Node<Target>,
    block: Node<ExprBlock>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Label(Option<Node<String>>);

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::If {
                val,
                t_block,
                f_block,
            } => match f_block.data.len() {
                0 => write!(f, "if {} {}", val, t_block),
                _ => write!(f, "if {} {} else {}", val, t_block, f_block),
            },
            Self::Match { val, arms } => write!(f, "match {} {{{}}}", val, Node::join(arms, ", ")),
            Self::For {
                label,
                tar,
                val,
                block,
            } => write!(f, "{}for {} in {} {}", label, tar, val, block),
            Self::While { label, val, block } => {
                write!(f, "{}while {} {}", label, val, block)
            }
            Self::Loop { label, block } => {
                write!(f, "{}loop {}", label, block)
            }
        }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(x) => write!(f, "'{} : ", x),
            None => write!(f, ""),
        }
    }
}

impl fmt::Display for MatchArm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.tar, self.block)
    }
}
