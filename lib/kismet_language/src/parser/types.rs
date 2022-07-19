use std::ops::Range;

use nom::{
    error::ParseError, AsBytes, Compare, Err, ExtendInto, FindSubstring, FindToken, InputIter,
    InputLength, InputTake, InputTakeAtPosition, Needed,
};

use crate::types::Span;

#[derive(Debug)]
pub struct Spanned<T> {
    pub span: Span,
    pub data: T,
}

impl<T: InputLength> Spanned<T> {
    pub fn new(data: T) -> Self {
        Spanned {
            span: Span::new(0..data.input_len()),
            data,
        }
    }
}

impl<T: AsBytes> AsBytes for Spanned<T> {
    fn as_bytes(&self) -> &[u8] {
        AsBytes::as_bytes(&self.data)
    }
}

impl<T, O> Compare<O> for Spanned<T>
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

impl<T: ExtendInto> ExtendInto for Spanned<T> {
    type Item = <T as ExtendInto>::Item;

    type Extender = <T as ExtendInto>::Extender;

    fn new_builder(&self) -> Self::Extender {
        self.data.new_builder()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        self.data.extend_into(acc)
    }
}

impl<T, O> FindSubstring<O> for Spanned<T>
where
    T: FindSubstring<O>,
{
    fn find_substring(&self, substr: O) -> Option<usize> {
        self.data.find_substring(substr)
    }
}

impl<T, O> FindToken<O> for Spanned<T>
where
    T: FindToken<O>,
{
    fn find_token(&self, token: O) -> bool {
        self.data.find_token(token)
    }
}

impl<T: InputIter> InputIter for Spanned<T> {
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

impl<T: InputLength> InputLength for Spanned<T> {
    fn input_len(&self) -> usize {
        self.data.input_len()
    }
}

impl<T: InputTake> InputTake for Spanned<T> {
    fn take(&self, count: usize) -> Self {
        Spanned {
            span: self.span.slice(..count),
            data: self.data.take(count),
        }
    }
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (lhs, rhs) = self.data.take_split(count);
        (
            Spanned {
                span: self.span.slice(..count),
                data: lhs,
            },
            Spanned {
                span: self.span.slice(count..),
                data: rhs,
            },
        )
    }
}

impl<'input, T: InputTakeAtPosition + InputLength> InputTakeAtPosition for Spanned<T> {
    type Item = <T as InputTakeAtPosition>::Item;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.data.split_at_position::<P, ()>(predicate) {
            Ok((lhs, rhs)) => {
                let loc = lhs.input_len();
                Ok((
                    Spanned {
                        span: self.span.slice(..loc),
                        data: lhs,
                    },
                    Spanned {
                        span: self.span.slice(loc..),
                        data: rhs,
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
            Ok((lhs, rhs)) => {
                let loc = lhs.input_len();
                Ok((
                    Spanned {
                        span: self.span.slice(..loc),
                        data: lhs,
                    },
                    Spanned {
                        span: self.span.slice(loc..),
                        data: rhs,
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
            Ok((lhs, rhs)) => {
                let loc = lhs.input_len();
                Ok((
                    Spanned {
                        span: self.span.slice(..loc),
                        data: lhs,
                    },
                    Spanned {
                        span: self.span.slice(loc..),
                        data: rhs,
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
            Ok((lhs, rhs)) => {
                let loc = lhs.input_len();
                Ok((
                    Spanned {
                        span: self.span.slice(..loc),
                        data: lhs,
                    },
                    Spanned {
                        span: self.span.slice(loc..),
                        data: rhs,
                    },
                ))
            }
            Err(_) => Err(Err::Incomplete(Needed::new(1))),
        }
    }
}
