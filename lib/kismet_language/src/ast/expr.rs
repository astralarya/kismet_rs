use std::fmt;

use crate::token::Token;

use super::{atom::Atom, node::Node};

#[derive(Debug, PartialEq)]
pub enum Expr<'input> {
    Stmts(Vec<Node<Expr<'input>>>),
    Op(Node<Expr<'input>>, Token<'input>, Node<Expr<'input>>),
    Unary(Token<'input>, Node<Expr<'input>>),
    Coefficient(Node<Atom<'input>>, Node<Expr<'input>>),
    Die(Node<Expr<'input>>),
    Attribute(Node<Expr<'input>>, Node<&'input str>),
    Atom(Atom<'input>),
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Stmts(val) => write!(f, "{}", Node::vec_to_string(&val, "\n")),
            Expr::Op(lhs, val, rhs) => {
                write!(f, "{}{}{}{}{}", lhs, val.space(), val, val.space(), rhs)
            }
            Expr::Unary(lhs, val) => write!(f, "{}{}{}", lhs, lhs.space(), val),
            Expr::Coefficient(lhs, rhs) => write!(f, "{}{}", lhs, rhs),
            Expr::Attribute(lhs, rhs) => write!(f, "{}.{}", lhs, rhs),
            Expr::Die(val) => match *val.kind {
                Expr::Atom(Atom::Integer(_))
                | Expr::Atom(Atom::Tuple(_))
                | Expr::Atom(Atom::ListDisplay(_)) => {
                    write!(f, "d{}", val)
                }
                _ => write!(f, "d({})", val),
            },
            Expr::Atom(val) => write!(f, "{}", val),
        }
    }
}
