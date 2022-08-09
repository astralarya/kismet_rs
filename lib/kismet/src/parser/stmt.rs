use nom::multi::{many0, many1, separated_list0};

use crate::ast::{Expr, ExprEnclosure};
use crate::types::{Node, Span};

use super::{expr, token_tag, Input, KResult, Token};

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
    expr(i)
}
