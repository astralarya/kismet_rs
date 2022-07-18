use std::fmt;

use crate::ast::Atom;
use crate::token::Token;
use crate::types::Span;

use super::node::Node;

#[derive(Debug, PartialEq)]
pub enum Expr<'input> {
    Stmts(Vec<Node<Expr<'input>>>),
    Comprehension(Node<Expr<'input>>, Vec<Node<Expr<'input>>>),
    CompFor(
        Node<Expr<'input>>,
        Node<Expr<'input>>,
        Option<Node<Expr<'input>>>,
    ),
    TargetList(Vec<Node<Atom<'input>>>),
    Op(Node<Expr<'input>>, Token<'input>, Node<Expr<'input>>),
    Unary(Token<'input>, Node<Expr<'input>>),
    Atom(Atom<'input>),
}

impl<'input> Node<Expr<'input>> {
    pub fn stmts((span, v): (Span, Vec<Node<Expr<'input>>>)) -> Node<Expr<'input>> {
        Node {
            span,
            kind: Box::new(Expr::Stmts(v)),
        }
    }

    pub fn comprehension(
        expr: Node<Expr<'input>>,
        iter: Vec<Node<Expr<'input>>>,
    ) -> Node<Expr<'input>> {
        Node {
            span: expr.span.clone() + Node::vec_to_span(&iter),
            kind: Box::new(Expr::Comprehension(expr, iter)),
        }
    }

    pub fn comp_for(
        span: Span,
        item: Node<Expr<'input>>,
        iter: Node<Expr<'input>>,
        ifnode: Option<Node<Expr<'input>>>,
    ) -> Node<Expr<'input>> {
        Node {
            span,
            kind: Box::new(Expr::CompFor(item, iter, ifnode)),
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
            Expr::Comprehension(e, i) => write!(f, "{} {}", e, Node::vec_to_string(&i, " ")),
            Expr::CompFor(item, iter, expr) => match expr {
                Some(node) => write!(f, "FOR {} IN {} IF {}", item, iter, node),
                None => write!(f, "FOR {} IN {}", item, iter),
            },
            Expr::TargetList(v) => write!(f, "{}", Node::vec_to_string(&v, ", ")),
            Expr::Op(left, op, right) => {
                match (op.enclose(&*left.kind), op.enclose(&*right.kind)) {
                    (true, true) => {
                        write!(f, "({}){}{}{}({})", left, op.space(), op, op.space(), right)
                    }
                    (true, false) => {
                        write!(f, "({}){}{}{}{}", left, op.space(), op, op.space(), right)
                    }
                    (false, true) => {
                        write!(f, "{}{}{}{}({})", left, op.space(), op, op.space(), right)
                    }
                    (false, false) => {
                        write!(f, "{}{}{}{}{}", left, op.space(), op, op.space(), right)
                    }
                }
            }
            Expr::Unary(op, right) => match op.enclose(&*right.kind) {
                true => write!(f, "{}{}({})", op, op.space(), right),
                false => write!(f, "{}{}{}", op, op.space(), right),
            },
            Expr::Atom(a) => write!(f, "{}", a),
        }
    }
}
