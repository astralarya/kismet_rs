use std::{
    cmp::{max, min},
    ops,
    ops::{Deref, Range},
};

extern crate overload;
use overload::overload;

pub type Integer = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct Span(pub Range<usize>);

impl Span {
    pub fn combine(lhs: Option<Span>, rhs: Option<Span>) -> Option<Span> {
        match (lhs, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(l), None) => Some(l.clone()),
            (None, Some(r)) => Some(r.clone()),
            (None, None) => None,
        }
    }

    pub fn combine_ref(lhs: Option<&Span>, rhs: Option<&Span>) -> Option<Span> {
        match (lhs, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(l), None) => Some(l.clone()),
            (None, Some(r)) => Some(r.clone()),
            (None, None) => None,
        }
    }
}

impl Deref for Span {
    type Target = Range<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

overload!((l: ?Span) + (r: ?Span) -> Span {
    Span(min(l.start, r.start)..max(l.end, r.end))
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
