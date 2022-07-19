use nom::{
    bytes::complete::{tag, take_while},
    error::Error,
    sequence::delimited,
    IResult, Parser,
};

use crate::{
    ast::Node,
    types::{Float, Integer},
};

#[derive(Clone, Debug)]
pub enum Token<'input> {
    DELIM,
    COMMA,
    COLON,
    SPREAD,
    String(String),
    Integer(Integer),
    Float(Float),
    Id(&'input str),
}

pub fn skip<'input>(i: Node<&'input str>) -> IResult<Node<&'input str>, Node<&'input str>> {
    let chars = " \t\r";
    take_while(move |c| chars.contains(c))(i)
}

pub fn token<'input, P>(
    parser: P,
    token: impl Fn(Node<&'input str>) -> Token<'input>,
) -> impl FnMut(Node<&'input str>) -> IResult<Node<&'input str>, Node<Token<'input>>>
where
    P: Parser<Node<&'input str>, Node<&'input str>, Error<Node<&'input str>>>,
{
    let mut curry = delimited(skip, parser, skip);
    move |i: Node<&'input str>| {
        let (tail, head) = curry(i)?;
        Ok((tail, Node::new(head.span, token(head))))
    }
}

pub fn delim<'input>(i: Node<&'input str>) -> IResult<Node<&'input str>, Node<Token<'input>>> {
    token(tag(";"), |_| Token::DELIM)(i)
}

pub fn comma<'input>(i: Node<&'input str>) -> IResult<Node<&'input str>, Node<Token<'input>>> {
    token(tag(","), |_| Token::COMMA)(i)
}
