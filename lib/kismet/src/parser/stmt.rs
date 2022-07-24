use nom::multi::{many0, many1};

use crate::{
    ast::Expr,
    types::{Node, Span},
};

use super::{expr, token_tag, Input, KResult, Token};

pub fn stmt<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, _) = many0(token_tag(Token::DELIM))(i)?;
    let (i, val) = expr(i)?;
    let (i, _) = many1(token_tag(Token::DELIM))(i)?;
    Ok((i, val))
}

pub fn stmts<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let i_start = i.span.start;
    let (i, mut val) = many0(stmt)(i)?;
    let (i, last) = expr(i)?;
    val.push(last);
    let i_range = i_start..i.span.start;
    Ok((
        i,
        Node::new(
            match Span::try_from(&val) {
                Ok(span) => span,
                Err(_) => Span::new(i_range),
            },
            Expr::Stmts(val),
        ),
    ))
}
