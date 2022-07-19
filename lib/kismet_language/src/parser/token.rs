use nom::{
    bytes::complete::{tag, take_while},
    sequence::delimited,
    IResult,
};

use crate::{
    ast::Node,
    types::{Float, Integer},
};

#[derive(Debug)]
pub enum Token<'input> {
    DELIM,
    String(String),
    Integer(Integer),
    Float(Float),
    Id(&'input str),
}

pub fn skip<'input>(i: Node<&'input str>) -> IResult<Node<&'input str>, Node<&'input str>> {
    let chars = " \t\r";
    take_while(move |c| chars.contains(c))(i)
}

pub fn delim<'input>(i: Node<&'input str>) -> IResult<Node<&'input str>, Node<Token<'input>>> {
    let (lhs, rhs) = delimited(skip, tag(";"), skip)(i)?;
    Ok((rhs, Node::new(lhs.span, Token::DELIM)))
}
