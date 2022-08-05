use std::fmt;

use crate::types::Node;

use super::{Atom, Expr, Range};

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    And(Node<Expr>, Node<Expr>),
    Or(Node<Expr>, Node<Expr>),
    Not(Node<Expr>),
    CompareBound {
        l_val: Node<Expr>,
        l_op: Node<OpEqs>,
        val: Node<Expr>,
        r_op: Node<OpEqs>,
        r_val: Node<Expr>,
    },
    Compare(Node<Expr>, Node<OpEqs>, Node<Expr>),
    Range(Range),
    Arith(Node<Expr>, Node<OpArith>, Node<Expr>),
    Unary(Node<OpArith>, Node<Expr>),
    Coefficient(Node<Atom>, Node<Expr>),
    Die(Node<Atom>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpEqs {
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpRange {
    RANGE,
    RANGEI,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpArith {
    ADD,
    SUB,
    MUL,
    DIV,
    IDIV,
    MOD,
    POW,
}

impl OpArith {
    pub fn space(&self) -> &'static str {
        match self {
            Self::POW | Self::MUL | Self::MOD => "",
            _ => " ",
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::And(lhs, rhs) => write!(f, "{} and {}", lhs, rhs),
            Self::Or(lhs, rhs) => write!(f, "{} or {}", lhs, rhs),
            Self::Not(val) => write!(f, "not {}", val),
            Self::CompareBound {
                l_val,
                l_op,
                val,
                r_op,
                r_val,
            } => write!(f, "{} {} {} {} {}", l_val, l_op, val, r_op, r_val),
            Self::Compare(lhs, op, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Self::Range(val) => write!(f, "{}", val),
            Self::Arith(lhs, op, rhs) => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    lhs,
                    op.data.space(),
                    op,
                    op.data.space(),
                    rhs
                )
            }
            Self::Unary(lhs, val) => write!(f, "{}{}", lhs, val),
            Self::Coefficient(lhs, rhs) => write!(f, "{}{}", lhs, rhs),
            Self::Die(val) => match *val.data {
                Atom::Id(_) => write!(f, "d({})", val),
                _ => write!(f, "d{}", val),
            },
        }
    }
}

impl fmt::Display for OpEqs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EQ => write!(f, "=="),
            Self::NE => write!(f, "!="),
            Self::LT => write!(f, "<"),
            Self::LE => write!(f, "<="),
            Self::GT => write!(f, ">"),
            Self::GE => write!(f, ">="),
        }
    }
}

impl fmt::Display for OpRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RANGE => write!(f, ".."),
            Self::RANGEI => write!(f, "..="),
        }
    }
}

impl fmt::Display for OpArith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ADD => write!(f, "+"),
            Self::SUB => write!(f, "-"),
            Self::MUL => write!(f, "*"),
            Self::DIV => write!(f, "/"),
            Self::IDIV => write!(f, "/%"),
            Self::MOD => write!(f, "%"),
            Self::POW => write!(f, "^"),
        }
    }
}
