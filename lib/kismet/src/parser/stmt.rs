use nom::{
    branch::alt,
    combinator::opt,
    multi::{many0, many1, separated_list0},
    Err,
};

use crate::ast::{Expr, ExprEnclosure, Stmt, Target};
use crate::types::{Node, ONode, Span};

use super::{expr, loop_label, token_tag, Error, ErrorKind, Input, KResult, Token};

pub fn stmt_block0(i: Input) -> KResult<Option<Node<Vec<Node<Expr>>>>> {
    let i_span = match Span::get0(i) {
        Some(x) => x,
        None => return Ok((i, None)),
    };
    let (i, _lhs) = many0(token_tag(Token::DELIM))(i)?;
    let (i, val) = separated_list0(many1(token_tag(Token::DELIM)), stmt)(i)?;
    let (i, _rhs) = many0(token_tag(Token::DELIM))(i)?;
    Ok((
        i,
        Some(Node::new(
            match Span::reduce(&val) {
                Some(x) => x,
                None => {
                    Span::new(i_span.start..Span::get0(i).map(|x| x.start).unwrap_or(i_span.end))
                }
            },
            val,
        )),
    ))
}

pub fn stmt_block1(i: Input) -> KResult<Node<Vec<Node<Expr>>>> {
    let (i, lhs) = many0(token_tag(Token::DELIM))(i)?;
    let (i, head) = stmt(i)?;
    let (i, _sep) = many1(token_tag(Token::DELIM))(i)?;
    let (i, mut val) = separated_list0(many1(token_tag(Token::DELIM)), stmt)(i)?;
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

pub fn stmt_enclosure(i: Input) -> KResult<Node<ExprEnclosure>> {
    let (i, lhs) = token_tag(Token::LBRACE)(i)?;
    let (i, val) = stmt_block0(i)?;
    let (i, rhs) = token_tag(Token::RBRACE)(i)?;
    match val {
        Some(val) => Ok((i, Node::new(lhs.span + rhs.span, ExprEnclosure(*val.data)))),
        None => Ok((i, Node::new(lhs.span + rhs.span, ExprEnclosure(vec![])))),
    }
}

pub fn stmt(i: Input) -> KResult<Node<Expr>> {
    alt((break_stmt, assignment_stmt))(i)
}

pub fn assignment_stmt(i: Input) -> KResult<Node<Expr>> {
    let (i, lhs) = expr(i)?;
    let (i, op) = opt(token_tag(Token::ASSIGN))(i)?;
    match op {
        Some(op) => match Node::<Target>::try_from(lhs) {
            Ok(lhs) => {
                let (i, rhs) = expr(i)?;
                Ok((i, Node::new(lhs.span + rhs.span, Expr::Assign(lhs, rhs))))
            }
            Err(_) => Err(Err::Failure(ONode::new(
                op.span,
                Error::Error(ErrorKind::Grammar),
            ))),
        },
        None => Ok((i, lhs)),
    }
}

pub fn break_stmt(i: Input) -> KResult<Node<Expr>> {
    let (i, op) = token_tag(Token::BREAK)(i)?;
    let (i, id) = opt(loop_label)(i)?;
    let (i, val) = expr(i)?;
    Ok((
        i,
        Node::new(op.span + val.span, Expr::Stmt(Stmt::Break { id, val })),
    ))
}
