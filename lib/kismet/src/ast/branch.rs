use std::fmt;

use crate::types::Node;

use super::{Expr, ExprEnclosure, Id, Match, Target};

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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Loop {
    pub id: Option<Node<Id>>,
    pub data: LoopKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoopKind {
    For {
        tar: Node<Target>,
        val: Node<Expr>,
        block: Node<ExprEnclosure>,
    },
    While {
        val: Node<Expr>,
        block: Node<ExprEnclosure>,
    },
    Loop {
        block: Node<ExprEnclosure>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchArm {
    pub tar: Node<Match>,
    pub block: Node<ExprEnclosure>,
}

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
            Self::Match { val, arms } => write!(f, "match {} {{{}}}", val, Node::join(arms, " ")),
        }
    }
}

impl fmt::Display for LoopKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::For { tar, val, block } => write!(f, "for {} in {} {}", tar, val, block),
            Self::While { val, block } => {
                write!(f, "while {} {}", val, block)
            }
            Self::Loop { block } => {
                write!(f, "loop {}", block)
            }
        }
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.id {
            Some(id) => write!(f, ":{}: {}", id, self.data),
            None => write!(f, "{}", self.data),
        }
    }
}

impl fmt::Display for MatchArm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.tar, self.block)
    }
}
