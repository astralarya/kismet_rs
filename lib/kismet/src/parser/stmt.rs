use nom::{
    combinator::all_consuming,
    multi::{many0, many1, separated_list0},
    Err,
};

use crate::{
    ast::Expr,
    types::{Node, ONode, Span},
};

use super::{expr, token_tag, ErrorKind, Input, KResult, Token};

pub fn expr_list0<'input>(i: Input<'input>) -> KResult<'input, Option<Node<Vec<Node<Expr>>>>> {
    let i_span = match Span::get0(i) {
        Some(x) => x,
        None => return Ok((i, None)),
    };
    let (i, _lhs) = many0(token_tag(Token::DELIM))(i)?;
    let (i, val) = separated_list0(many1(token_tag(Token::DELIM)), expr)(i)?;
    let (i, _rhs) = many0(token_tag(Token::DELIM))(i)?;
    Ok((
        i,
        Some(Node::new(
            Span::reduce(&val).unwrap_or(Span::new(
                i_span.start..Span::get0(i).map(|x| x.start).unwrap_or(i_span.end),
            )),
            val,
        )),
    ))
}

pub fn expr_list1<'input>(i: Input<'input>) -> KResult<'input, Node<Vec<Node<Expr>>>> {
    let (i, lhs) = many0(token_tag(Token::DELIM))(i)?;
    let (i, head) = expr(i)?;
    let (i, _sep) = many1(token_tag(Token::DELIM))(i)?;
    let (i, mut val) = separated_list0(many1(token_tag(Token::DELIM)), expr)(i)?;
    let (i, rhs) = many0(token_tag(Token::DELIM))(i)?;
    let head_span = head.span;
    val.insert(0, head);
    Ok((
        i,
        Node::new(
            head_span + Span::reduce(&val) + Span::reduce_ref(&lhs) + Span::reduce_ref(&rhs),
            val,
        ),
    ))
}

pub fn start<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let i_span = match Span::get0(i) {
        Some(x) => x,
        None => return Err(Err::Failure(ONode::new(None, ErrorKind::Eof))),
    };
    let (i, val) = all_consuming(expr_list0)(i)?;
    match val {
        Some(val) => Ok((i, Node::new(val.span, Expr::Stmts(*val.data)))),
        None => Err(Err::Failure(ONode::new(i_span, ErrorKind::Eof))),
    }
}
