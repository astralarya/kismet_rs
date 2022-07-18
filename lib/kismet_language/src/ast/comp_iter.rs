use std::fmt;

use crate::types::Span;

use super::{Expr, Node};

#[derive(Debug, PartialEq)]
pub enum CompIter<'input> {
    For(Node<Expr<'input>>, Node<Expr<'input>>),
    If(Node<Expr<'input>>),
}

impl<'input> Node<CompIter<'input>> {
    pub fn comp_for(
        span: Span,
        for_expr: Node<Expr<'input>>,
        in_expr: Node<Expr<'input>>,
    ) -> Node<CompIter<'input>> {
        Node {
            span,
            kind: Box::new(CompIter::For(for_expr, in_expr)),
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
            CompIter::For(t, i) => write!(f, "for {} in {}", t, i),
            CompIter::If(node) => write!(f, "if {}", node),
        }
    }
}
