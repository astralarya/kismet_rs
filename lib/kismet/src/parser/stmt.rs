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
    let (i, mut val) = many0(stmt)(i)?;
    let (i, last) = expr(i)?;
    val.push(last);
    Ok((i, Node::new(Span::from_iter(&val), Expr::Stmts(val))))
}
