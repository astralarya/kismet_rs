use std::{fmt, ops::Deref};

use crate::types::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
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

impl<T> From<Node<T>> for Span {
    fn from(item: Node<T>) -> Self {
        item.span
    }
}

impl<S, T> BaseNode<S, T> {
    pub fn new<R>(range: R, val: T) -> BaseNode<S, T>
    where
        S: From<R>,
    {
        BaseNode {
            span: S::from(range),
            data: Box::new(val),
        }
    }

    pub fn convert<U>(fun: impl Fn(U) -> T, val: BaseNode<S, U>) -> Self {
        BaseNode::new(val.span, fun(*val.data))
    }

    pub fn convert_from<U>(val: BaseNode<S, U>) -> Self
    where
        T: From<U>,
    {
        BaseNode::new(val.span, T::from(*val.data))
    }

    pub fn try_convert<U, E>(
        fun: impl Fn(U) -> Result<T, E>,
        val: BaseNode<S, U>,
    ) -> Result<Self, E> {
        Ok(BaseNode::new(val.span, fun(*val.data)?))
    }

    pub fn try_convert_from<U, E>(val: BaseNode<S, U>) -> Result<Self, E>
    where
        T: TryFrom<U, Error = E>,
    {
        Ok(BaseNode::new(val.span, T::try_from(*val.data)?))
    }
}

impl<S, T> Deref for BaseNode<S, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}

impl<N, T: std::fmt::Display> BaseNode<N, T> {
    pub fn join(nodes: &[Node<T>], delim: &'static str) -> String {
        nodes
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join(delim)
    }

    pub fn join1(nodes: &[Node<T>], delim: &'static str, delim1: &'static str) -> String {
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
