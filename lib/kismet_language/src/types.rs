use std::ops::{Add, Deref, Range};

pub type Integer = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct Span(pub Range<usize>);
pub struct SpanVec(pub Vec<Span>);

impl Span {
    pub fn combine(lhs: Option<&Span>, rhs: Option<&Span>) -> Option<Span> {
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

impl<'a> Add for &'a Span {
    type Output = Span;
    fn add(self, rhs: Self) -> Self::Output {
        Span(self.start..rhs.end)
    }
}

impl<'a> Add<Span> for &'a Span {
    type Output = Span;
    fn add(self, rhs: Span) -> Self::Output {
        self + &rhs
    }
}

impl Add for Span {
    type Output = Span;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<'a> Add<&'a Span> for Span {
    type Output = Span;
    fn add(self, rhs: &Self) -> Self::Output {
        &self + rhs
    }
}

impl<'a> Add<Option<Span>> for &'a Span {
    type Output = Span;
    fn add(self, rhs: Option<Span>) -> Self::Output {
        match rhs {
            Some(span) => self + span,
            None => self.clone(),
        }
    }
}

impl Add<Option<Span>> for Span {
    type Output = Span;
    fn add(self, rhs: Option<Span>) -> Self::Output {
        &self + rhs
    }
}

impl<'a> Add<&'a Span> for Option<Span> {
    type Output = Span;
    fn add(self, rhs: &'a Span) -> Self::Output {
        rhs + self
    }
}

impl Add<Span> for Option<Span> {
    type Output = Span;
    fn add(self, rhs: Span) -> Self::Output {
        &rhs + self
    }
}

impl SpanVec {
    pub fn to_span(&self) -> Option<Span> {
        Span::combine(self.first(), self.last())
    }
}

impl Deref for SpanVec {
    type Target = Vec<Span>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Add for &'a SpanVec {
    type Output = Option<Span>;
    fn add(self, rhs: Self) -> Self::Output {
        Span::combine(self.first(), rhs.last())
    }
}

impl<'a> Add<SpanVec> for &'a SpanVec {
    type Output = Option<Span>;
    fn add(self, rhs: SpanVec) -> Self::Output {
        self + &rhs
    }
}

impl Add for SpanVec {
    type Output = Option<Span>;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<'a> Add<&'a SpanVec> for SpanVec {
    type Output = Option<Span>;
    fn add(self, rhs: &'a SpanVec) -> Self::Output {
        &self + rhs
    }
}
