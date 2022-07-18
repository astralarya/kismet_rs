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
    Atom(Atom<'input>),
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Stmts(nodes) => write!(f, "{}", Node::vec_to_string(&nodes, "\n")),
            Expr::Op(left, op, right) => {
                write!(f, "{}{}{}{}{}", left, op.space(), op, op.space(), right)
            }
            Expr::Unary(op, right) => write!(f, "{}{}{}", op, op.space(), right),
            Expr::Coefficient(l, r) => write!(f, "{}{}", l, r),
            Expr::Die(node) => match *node.kind {
                Expr::Atom(Atom::Integer(_))
                | Expr::Atom(Atom::Tuple(_))
                | Expr::Atom(Atom::ListDisplay(_)) => {
                    write!(f, "d{}", node)
                }
                _ => write!(f, "d({})", node),
            },
            Expr::Atom(a) => write!(f, "{}", a),
        }
    }
}
