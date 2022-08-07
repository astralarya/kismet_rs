use std::{
    cmp::{max, min},
    ops::{self, Index, Range, RangeFrom, RangeFull, RangeTo},
};

use nom::Err;
use overload::overload;

use crate::parser::ErrorKind;

use super::ONode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(input: Range<usize>) -> Self {
        Span::new(input.start..input.end)
    }
}

impl<'input> From<&'input str> for Span {
    fn from(input: &'input str) -> Self {
        Span::new(0..input.len())
    }
}

impl From<String> for Span {
    fn from(input: String) -> Self {
        Span::new(0..input.len())
    }
}

impl Span {
    pub fn new(range: Range<usize>) -> Self {
        Span {
            start: range.start,
            end: range.end,
        }
    }

    pub fn slice<T: SliceSpan>(&self, range: T) -> Self {
        range.slice_span(self)
    }

    pub fn option<'input, T>(val: &'input Option<T>) -> Option<Span>
    where
        Span: From<&'input T>,
    {
        val.as_ref().map(Span::from)
    }

    pub fn option_ref<'input, T>(val: &'input Option<&'input T>) -> Option<Span>
    where
        Span: From<&'input T>,
    {
        val.as_ref().map(|val| Span::from(val))
    }

    pub fn add_option(lhs: Option<Span>, rhs: Option<Span>) -> Option<Span> {
        match (lhs, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }

    pub fn add_option_ref(lhs: Option<&Span>, rhs: Option<&Span>) -> Option<Span> {
        match (lhs, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(l), None) => Some(*l),
            (None, Some(r)) => Some(*r),
            (None, None) => None,
        }
    }

    pub fn reduce<'input, T>(vec: &'input [T]) -> Option<Span>
    where
        Span: From<&'input T>,
    {
        vec.iter().map(Span::from).reduce(|acc, next| acc + next)
    }

    pub fn reduce_ok<'input, T>(vec: &'input [T]) -> Result<Span, Err<ONode<ErrorKind>>>
    where
        Span: From<&'input T>,
    {
        Self::reduce(vec).ok_or_else(|| Err::Failure(ONode::new(None, ErrorKind::Runtime)))
    }

    pub fn reduce_ref<'input, T>(vec: &'input [&'input T]) -> Option<Span>
    where
        Span: From<&'input T>,
    {
        vec.iter()
            .map(|x| Span::from(x))
            .reduce(|acc, next| acc + next)
    }

    pub fn reduce_ref_ok<'input, T>(vec: &'input [&'input T]) -> Result<Span, Err<ONode<ErrorKind>>>
    where
        Span: From<&'input T>,
    {
        Self::reduce_ref(vec).ok_or_else(|| Err::Failure(ONode::new(None, ErrorKind::Runtime)))
    }

    pub fn get0<'input, T>(input: &'input [T]) -> Option<Span>
    where
        Span: From<&'input T>,
    {
        input.get(0).map(Span::from)
    }
}

pub trait SliceSpan {
    fn slice_span(&self, span: &Span) -> Span;
}

impl SliceSpan for usize {
    fn slice_span(&self, span: &Span) -> Span {
        Span::new(span.start + self..span.start + self)
    }
}

impl SliceSpan for Range<usize> {
    fn slice_span(&self, span: &Span) -> Span {
        Span::new(span.start + self.start..span.start + self.end)
    }
}

impl SliceSpan for RangeFrom<usize> {
    fn slice_span(&self, span: &Span) -> Span {
        Span::new(span.start + self.start..span.end)
    }
}

impl SliceSpan for RangeTo<usize> {
    fn slice_span(&self, span: &Span) -> Span {
        Span::new(span.start..span.start + self.end)
    }
}

impl Index<RangeFull> for Span {
    type Output = Span;

    fn index(&self, _: RangeFull) -> &Self::Output {
        self
    }
}

overload!((l: ?Span) + (r: ?Span) -> Span {
    Span::new(min(l.start, r.start)..max(l.end, r.end))
});

overload!((l: ?Span) + (r: ?Option<Span>) -> Span {
    match r {
        Some(span) => l + span,
        None => l.clone(),
    }
});

overload!((l: ?Option<Span>) + (r: ?Span) -> Span {
    r + l
});

overload!((l: ?Span) + (r: ?Option<&Span>) -> Span {
    match r {
        Some(span) => l + span.clone(),
        None => l.clone(),
    }
});

overload!((l: ?Option<&Span>) + (r: ?Span) -> Span {
    r + l
});
