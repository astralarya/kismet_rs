use std::{fmt, ops::Deref};

use crate::types::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct BaseNode<N, T> {
    pub span: N,
    pub data: Box<T>,
}

pub type Node<T> = BaseNode<Span, T>;
pub type ONode<T> = BaseNode<Option<Span>, T>;

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

impl<N, T> BaseNode<N, T> {
    pub fn new<S>(range: S, val: T) -> BaseNode<N, T>
    where
        N: From<S>,
    {
        BaseNode {
            span: N::from(range),
            data: Box::new(val),
        }
    }
}

impl<N, T> Deref for BaseNode<N, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<N, T: std::fmt::Display> BaseNode<N, T> {
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

impl<N, T: std::fmt::Display> fmt::Display for BaseNode<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
