use std::fmt;

use crate::types::Node;

use super::{Atom, OpArith, OpEqs, Primary, Range, TargetList};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Stmts(Vec<Node<Expr>>),
    Assign(Node<TargetList>, Node<Expr>),
    And(Node<Expr>, Node<Expr>),
    Or(Node<Expr>, Node<Expr>),
    Not(Node<Expr>),
    CompareBound {
        val: Node<Expr>,
        l_op: OpEqs,
        l_val: Node<Expr>,
        r_op: OpEqs,
        r_val: Node<Expr>,
    },
    Compare(Node<Expr>, OpEqs, Node<Expr>),
    Range(Range),
    Arith(Node<Expr>, OpArith, Node<Expr>),
    Unary(OpArith, Node<Expr>),
    Coefficient(Node<Atom>, Node<Expr>),
    Die(Node<Atom>),
    Primary(Primary),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Stmts(val) => write!(f, "{}", Node::vec_to_string(&val, "\n")),
            Expr::Assign(lhs, rhs) => write!(f, "{} := {}", lhs, rhs),
            Expr::And(lhs, rhs) => write!(f, "{} and {}", lhs, rhs),
            Expr::Or(lhs, rhs) => write!(f, "{} or {}", lhs, rhs),
            Expr::Not(val) => write!(f, "not {}", val),
            Expr::CompareBound {
                val,
                l_op,
                l_val,
                r_op,
                r_val,
            } => write!(f, "{} {} {} {} {}", l_val, l_op, val, r_op, r_val),
            Expr::Compare(lhs, op, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Expr::Range(val) => write!(f, "{}", val),
            Expr::Arith(lhs, op, rhs) => {
                write!(f, "{}{}{}{}{}", lhs, op.space(), op, op.space(), rhs)
            }
            Expr::Unary(lhs, val) => write!(f, "{}{}{}", lhs, lhs.space(), val),
            Expr::Coefficient(lhs, rhs) => write!(f, "{}{}", lhs, rhs),
            Expr::Die(val) => match *val.data {
                Atom::Id(_) => write!(f, "d({})", val),
                _ => write!(f, "d{}", val),
            },
            Expr::Primary(val) => write!(f, "{}", val),
        }
    }
}
