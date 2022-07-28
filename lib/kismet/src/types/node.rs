use std::fmt;

use nom::{
    error::ParseError, AsBytes, Compare, Err, ExtendInto, FindSubstring, FindToken, InputIter,
    InputLength, InputTake, InputTakeAtPosition, Needed,
};

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

impl<T> From<Node<T>> for Span {
    fn from(item: Node<T>) -> Self {
        item.span
    }
}

impl<T> From<&Node<T>> for Span {
    fn from(item: &Node<T>) -> Self {
        item.span
    }
}

impl<T: std::fmt::Display> Node<T> {
    pub fn vec_to_string(nodes: &Vec<Node<T>>, delim: &'static str) -> String {
        nodes
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join(delim)
    }

    pub fn vec_to_string1(
        nodes: &Vec<Node<T>>,
        delim: &'static str,
        delim1: &'static str,
    ) -> String {
        format!(
            "{}{}",
            Self::vec_to_string(nodes, delim),
            if nodes.len() == 1 { delim1 } else { "" }
        )
    }
}

impl<T: std::fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T: AsBytes> AsBytes for Node<T> {
    fn as_bytes(&self) -> &[u8] {
        AsBytes::as_bytes(&*self.data)
    }
}

impl<T, O> Compare<O> for Node<T>
where
    T: Compare<O>,
{
    fn compare(&self, t: O) -> nom::CompareResult {
        self.data.compare(t)
    }
    fn compare_no_case(&self, t: O) -> nom::CompareResult {
        self.data.compare_no_case(t)
    }
}

impl<T: ExtendInto> ExtendInto for Node<T> {
    type Item = <T as ExtendInto>::Item;

    type Extender = <T as ExtendInto>::Extender;

    fn new_builder(&self) -> Self::Extender {
        self.data.new_builder()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        self.data.extend_into(acc)
    }
}

impl<T, O> FindSubstring<O> for Node<T>
where
    T: FindSubstring<O>,
{
    fn find_substring(&self, substr: O) -> Option<usize> {
        self.data.find_substring(substr)
    }
}

impl<T, O> FindToken<O> for Node<T>
where
    T: FindToken<O>,
{
    fn find_token(&self, token: O) -> bool {
        self.data.find_token(token)
    }
}

impl<T: InputIter> InputIter for Node<T> {
    type Item = <T as InputIter>::Item;

    type Iter = <T as InputIter>::Iter;

    type IterElem = <T as InputIter>::IterElem;

    fn iter_indices(&self) -> Self::Iter {
        self.data.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.data.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.data.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.data.slice_index(count)
    }
}

impl<T: InputLength> InputLength for Node<T> {
    fn input_len(&self) -> usize {
        self.data.input_len()
    }
}

impl<T: InputTake + InputLength> InputTake for Node<T> {
    fn take(&self, count: usize) -> Self {
        let start = self.span.len() - self.data.input_len();
        Node {
            span: self.span.slice(start..start + count),
            data: Box::new(self.data.take(count)),
        }
    }
    fn take_split(&self, count: usize) -> (Self, Self) {
        let start = self.span.len() - self.data.input_len();
        let (suffix, prefix) = self.data.take_split(count);
        (
            Node {
                span: self.span.slice(start + count..),
                data: Box::new(suffix),
            },
            Node {
                span: self.span.slice(start..start + count),
                data: Box::new(prefix),
            },
        )
    }
}

impl<'input, T: InputTakeAtPosition + InputLength> InputTakeAtPosition for Node<T> {
    type Item = <T as InputTakeAtPosition>::Item;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.data.split_at_position::<P, ()>(predicate) {
            Ok((suffix, prefix)) => {
                let start = self.span.len() - self.data.input_len();
                let count = prefix.input_len();
                Ok((
                    Node {
                        span: self.span.slice(start + count..),
                        data: Box::new(suffix),
                    },
                    Node {
                        span: self.span.slice(start..start + count),
                        data: Box::new(prefix),
                    },
                ))
            }
            Err(_) => Err(Err::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position1<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.data.split_at_position1::<P, ()>(predicate, e) {
            Ok((suffix, prefix)) => {
                let start = self.span.len() - self.data.input_len();
                let count = prefix.input_len();
                Ok((
                    Node {
                        span: self.span.slice(start + count..),
                        data: Box::new(suffix),
                    },
                    Node {
                        span: self.span.slice(start..start + count),
                        data: Box::new(prefix),
                    },
                ))
            }
            Err(_) => Err(Err::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.data.split_at_position_complete::<P, ()>(predicate) {
            Ok((suffix, prefix)) => {
                let start = self.span.len() - self.data.input_len();
                let count = prefix.input_len();
                Ok((
                    Node {
                        span: self.span.slice(start + count..),
                        data: Box::new(suffix),
                    },
                    Node {
                        span: self.span.slice(start..start + count),
                        data: Box::new(prefix),
                    },
                ))
            }
            Err(_) => Err(Err::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position1_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.data.split_at_position1_complete::<P, ()>(predicate, e) {
            Ok((suffix, prefix)) => {
                let start = self.span.len() - self.data.input_len();
                let count = prefix.input_len();
                Ok((
                    Node {
                        span: self.span.slice(start + count..),
                        data: Box::new(suffix),
                    },
                    Node {
                        span: self.span.slice(start..start + count),
                        data: Box::new(prefix),
                    },
                ))
            }
            Err(_) => Err(Err::Incomplete(Needed::new(1))),
        }
    }
}
