use std::ops::{Add, Deref, Range};

pub type Integer = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct Span(pub Range<usize>);

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
        Span(self.start..rhs.end)
    }
}

impl Add for Span {
    type Output = Span;
    fn add(self, rhs: Self) -> Self::Output {
        Span(self.start..rhs.end)
    }
}

impl<'a> Add<&'a Span> for Span {
    type Output = Span;
    fn add(self, rhs: &Self) -> Self::Output {
        Span(self.start..rhs.end)
    }
}

impl Add<Option<Span>> for Span {
    type Output = Span;
    fn add(self, rhs: Option<Span>) -> Self::Output {
        match rhs {
            Some(span) => self + span,
            None => self.clone(),
        }
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

impl Add<Span> for Option<Span> {
    type Output = Span;
    fn add(self, rhs: Span) -> Self::Output {
        match self {
            Some(span) => &span + rhs,
            None => rhs,
        }
    }
}

impl<'a> Add<&'a Span> for Option<Span> {
    type Output = Span;
    fn add(self, rhs: &'a Span) -> Self::Output {
        match self {
            Some(span) => &span + rhs,
            None => rhs.clone(),
        }
    }
}

pub mod span {
    type Span = super::Span;

    fn merge(lhs: Option<Span>, rhs: Option<Span>) -> Option<Span> {
        match (lhs, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }
}
