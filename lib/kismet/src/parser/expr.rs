use nom::{
    branch::alt,
    combinator::opt,
    multi::{many0, many1, separated_list0},
    Err,
};

use crate::ast::{Expr, Target};
use crate::types::{Node, ONode, Span};

use super::{or_test, token_tag, ErrorKind, Input, KResult, Token};

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

pub fn expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    assignment_expr(i)
}

pub fn assignment_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, lhs) = conditional_expr(i)?;
    let (i, op) = opt(token_tag(Token::ASSIGNE))(i)?;
    match op {
        Some(op) => match Node::<Target>::try_from(lhs) {
            Ok(lhs) => {
                let (i, rhs) = conditional_expr(i)?;
                Ok((i, Node::new(lhs.span + rhs.span, Expr::Assign(lhs, rhs))))
            }
            Err(_) => Err(Err::Failure(ONode::new(op.span, ErrorKind::Grammar))),
        },
        None => Ok((i, lhs)),
    }
}

pub fn conditional_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    alt((
        if_expr,
        match_expr,
        for_expr,
        while_expr,
        loop_expr,
        lambda_expr,
    ))(i)
}

pub fn if_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    lambda_expr(i)
}

pub fn match_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    lambda_expr(i)
}

pub fn for_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    lambda_expr(i)
}

pub fn while_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    lambda_expr(i)
}

pub fn loop_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    lambda_expr(i)
}

pub fn lambda_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    or_test(i)
}
