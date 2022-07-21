use std::{
    cmp::{max, min},
    ops::{self, Index, Range, RangeFrom, RangeFull, RangeTo},
};

use num_complex::Complex;
use overload::overload;

pub type Integer = i32;
pub type Float = f32;
pub type Imaginary = Complex<f32>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl<'input> From<Range<usize>> for Span {
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
        range.slice_span(&self)
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn add_option(lhs: Option<Span>, rhs: Option<Span>) -> Option<Span> {
        match (lhs, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(l), None) => Some(l.clone()),
            (None, Some(r)) => Some(r.clone()),
            (None, None) => None,
        }
    }

    pub fn add_option_ref(lhs: Option<&Span>, rhs: Option<&Span>) -> Option<Span> {
        match (lhs, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(l), None) => Some(l.clone()),
            (None, Some(r)) => Some(r.clone()),
            (None, None) => None,
        }
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
        &self
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
