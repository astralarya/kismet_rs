use nom::{
    combinator::all_consuming,
    multi::{many0, many1, separated_list0},
};

use crate::{
    ast::Expr,
    types::{Node, Span},
};

use super::{expr, token_tag, Input, KResult, Token};

pub fn expr_list0<'input>(i: Input<'input>) -> KResult<'input, Node<Vec<Node<Expr>>>> {
    let (i, _lhs) = many0(token_tag(Token::DELIM))(i)?;
    let (i, val) = separated_list0(many1(token_tag(Token::DELIM)), expr)(i)?;
    let (i, _rhs) = many0(token_tag(Token::DELIM))(i)?;
    Ok((i, Node::new(Span::from_iter(&val), val)))
}

pub fn expr_list1<'input>(i: Input<'input>) -> KResult<'input, Node<Vec<Node<Expr>>>> {
    let (i, lhs) = many0(token_tag(Token::DELIM))(i)?;
    let (i, head) = expr(i)?;
    let (i, _sep) = many1(token_tag(Token::DELIM))(i)?;
    let (i, mut val) = separated_list0(many1(token_tag(Token::DELIM)), expr)(i)?;
    let (i, rhs) = many0(token_tag(Token::DELIM))(i)?;
    val.insert(0, head);
    Ok((
        i,
        Node::new(Span::from_iter(lhs) + Span::from_iter(rhs), val),
    ))
}

pub fn start<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, val) = all_consuming(expr_list0)(i)?;
    Ok((i, Node::new(val.span, Expr::Stmts(*val.data))))
}
