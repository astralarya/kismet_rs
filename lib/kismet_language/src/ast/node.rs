use std::fmt;

use crate::types::Span;

#[derive(Debug, PartialEq)]
pub struct Node<Kind> {
    pub span: Span,
    pub kind: Box<Kind>,
}

impl<T: std::fmt::Display> Node<T> {
    pub fn vec_to_string(nodes: &Vec<Node<T>>, delim: &'static str) -> String {
        nodes
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join(delim)
    }

    pub fn vec_to_span(v: &Vec<Node<T>>) -> Option<Span> {
        Span::reduce(&mut v.iter().map(|x| x.span.clone()))
    }
}

impl<T: std::fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
