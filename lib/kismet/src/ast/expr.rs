use std::fmt;

use crate::parser::Token;

use super::{Atom, Primary};
use crate::types::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Stmts(Vec<Node<Expr>>),
    Op(Node<Expr>, Node<Token>, Node<Expr>),
    Unary(Node<Token>, Node<Expr>),
    Coefficient(Node<Atom>, Node<Expr>),
    Die(Node<Atom>),
    Primary(Primary),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Stmts(val) => write!(f, "{}", Node::vec_to_string(&val, "\n")),
            Expr::Op(lhs, val, rhs) => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    lhs,
                    val.data.space(),
                    val,
                    val.data.space(),
                    rhs
                )
            }
            Expr::Unary(lhs, val) => write!(f, "{}{}{}", lhs, lhs.data.space(), val),
            Expr::Coefficient(lhs, rhs) => write!(f, "{}{}", lhs, rhs),
            Expr::Die(val) => match *val.data {
                Atom::Id(_) => write!(f, "d({})", val),
                _ => write!(f, "d{}", val),
            },
            Expr::Primary(val) => write!(f, "{}", val),
        }
    }
}
