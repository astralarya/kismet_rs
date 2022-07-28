use std::fmt;

use crate::types::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub span: Span,
    pub data: Box<T>,
}

impl<T> Node<T> {
    pub fn new<S>(range: S, val: T) -> Node<T>
    where
        Span: From<S>,
    {
        Node {
            span: Span::from(range),
            data: Box::new(val),
        }
    }
}

impl<T> From<T> for Node<T>
where
    Span: From<T>,
    T: Copy,
{
    fn from(input: T) -> Self {
        Node::new(input, input)
    }
}

impl<T> From<&Node<T>> for Span {
    fn from(item: &Node<T>) -> Self {
        item.span
    }
}

impl<T: std::fmt::Display> Node<T> {
    pub fn join(nodes: &Vec<Node<T>>, delim: &'static str) -> String {
        nodes
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join(delim)
    }

    pub fn join1(nodes: &Vec<Node<T>>, delim: &'static str, delim1: &'static str) -> String {
        format!(
            "{}{}",
            Self::join(nodes, delim),
            if nodes.len() == 1 { delim1 } else { "" }
        )
    }
}

impl<T: std::fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
