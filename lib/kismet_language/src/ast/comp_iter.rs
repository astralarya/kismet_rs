use std::fmt;

use crate::types::Span;

use super::{Expr, Node, Target};

#[derive(Debug, PartialEq)]
pub enum CompIter<'input> {
    For {
        target: Vec<Node<Target<'input>>>,
        expr: Node<Expr<'input>>,
    },
    If(Node<Expr<'input>>),
}

impl<'input> Node<CompIter<'input>> {
    pub fn comp_for(
        span: Span,
        target: Vec<Node<Target<'input>>>,
        expr: Node<Expr<'input>>,
    ) -> Node<CompIter<'input>> {
        Node {
            span,
            kind: Box::new(CompIter::For { target, expr }),
        }
    }

    pub fn comp_if(span: Span, expr: Node<Expr<'input>>) -> Node<CompIter<'input>> {
        Node {
            span,
            kind: Box::new(CompIter::If(expr)),
        }
    }
}

impl fmt::Display for CompIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            CompIter::For { target, expr } => {
                write!(f, "for {} in {}", Node::vec_to_string(&target, ", "), expr)
            }
            CompIter::If(node) => write!(f, "if {}", node),
        }
    }
}
