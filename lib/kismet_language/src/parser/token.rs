use nom::{
    bytes::complete::{tag, take_while},
    sequence::delimited,
    IResult,
};

use crate::types::{Float, Integer};

use super::Spanned;

#[derive(Debug)]
pub enum Token<'input> {
    DELIM,
    String(String),
    Integer(Integer),
    Float(Float),
    Id(&'input str),
}

pub fn skip<'input>(
    i: Spanned<&'input str>,
) -> IResult<Spanned<&'input str>, Spanned<&'input str>> {
    let chars = " \t\r";
    take_while(move |c| chars.contains(c))(i)
}

pub fn delim<'input>(
    i: Spanned<&'input str>,
) -> IResult<Spanned<&'input str>, Spanned<Token<'input>>> {
    let (lhs, rhs) = delimited(skip, tag(";"), skip)(i)?;
    Ok((
        rhs,
        Spanned {
            span: lhs.span,
            data: Token::DELIM,
        },
    ))
}
