use std::fmt;

use crate::ast::{Atom, Node};
use crate::token::Token;
use crate::types::Span;

#[derive(Debug, PartialEq)]
pub enum Expr<'input> {
    Stmts(Vec<Node<Expr<'input>>>),
    TargetList(Vec<Node<Atom<'input>>>),
    Op(Node<Expr<'input>>, Token<'input>, Node<Expr<'input>>),
    Unary(Token<'input>, Node<Expr<'input>>),
    Coefficient(Node<Atom<'input>>, Node<Expr<'input>>),
    Die(Node<Expr<'input>>),
    Atom(Atom<'input>),
}

impl<'input> Node<Expr<'input>> {
    pub fn stmts((span, v): (Span, Vec<Node<Expr<'input>>>)) -> Node<Expr<'input>> {
        Node {
            span,
            kind: Box::new(Expr::Stmts(v)),
        }
    }

    pub fn target_list((span, vector): (Span, Vec<Node<Atom<'input>>>)) -> Node<Expr<'input>> {
        Node {
            span,
            kind: Box::new(Expr::TargetList(vector)),
        }
    }

    pub fn op(
        l: Node<Expr<'input>>,
        o: Token<'input>,
        r: Node<Expr<'input>>,
    ) -> Node<Expr<'input>> {
        Node {
            span: l.span.clone() + r.span.clone(),
            kind: Box::new(Expr::Op(l, o, r)),
        }
    }

    pub fn unary(o: Token<'input>, r: Node<Expr<'input>>) -> Node<Expr<'input>> {
        Node {
            span: o.span.clone() + r.span.clone(),
            kind: Box::new(Expr::Unary(o, r)),
        }
    }

    pub fn coefficient(l: Node<Atom<'input>>, r: Node<Expr<'input>>) -> Node<Expr<'input>> {
        Node {
            span: l.span.clone() + r.span.clone(),
            kind: Box::new(Expr::Coefficient(l, r)),
        }
    }

    pub fn die(t: Token<'input>, r: Node<Expr<'input>>) -> Node<Expr<'input>> {
        Node {
            span: t.span + r.span.clone(),
            kind: Box::new(Expr::Die(r)),
        }
    }

    pub fn atom(a: Node<Atom<'input>>) -> Node<Expr<'input>> {
        Node {
            span: a.span,
            kind: Box::new(Expr::Atom(*a.kind)),
        }
    }
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Stmts(nodes) => write!(f, "{}", Node::vec_to_string(&nodes, "\n")),
            Expr::TargetList(v) => write!(f, "{}", Node::vec_to_string(&v, ", ")),
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
