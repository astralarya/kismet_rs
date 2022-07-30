use nom::{
    branch::alt,
    combinator::opt,
    multi::{many0, many1, separated_list0},
    sequence::preceded,
    Err,
};

use crate::ast::{Branch, Expr, ExprEnclosure, MatchArm, MatchBlock, Target};
use crate::types::{Node, ONode, Span};

use super::{or_test, target_match, token_tag, ErrorKind, Input, KResult, Token};

pub fn expr_block0<'input>(i: Input<'input>) -> KResult<'input, Option<Node<Vec<Node<Expr>>>>> {
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

pub fn expr_block1<'input>(i: Input<'input>) -> KResult<'input, Node<Vec<Node<Expr>>>> {
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

pub fn expr_enclosure<'input>(i: Input<'input>) -> KResult<'input, Node<ExprEnclosure>> {
    let (i, lhs) = token_tag(Token::LBRACE)(i)?;
    let (i, val) = expr_block0(i)?;
    let (i, rhs) = token_tag(Token::RBRACE)(i)?;
    match val {
        Some(val) => Ok((i, Node::new(lhs.span + rhs.span, ExprEnclosure(*val.data)))),
        None => Ok((i, Node::new(lhs.span + rhs.span, ExprEnclosure(vec![])))),
    }
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
    let (i, lhs) = token_tag(Token::IF)(i)?;
    let (i, val) = or_test(i)?;
    let (i, t_block) = expr_enclosure(i)?;
    let (i, f_block) = opt(preceded(token_tag(Token::ELSE), expr_enclosure))(i)?;
    match f_block {
        Some(f_block) => Ok((
            i,
            Node::new(
                lhs.span + f_block.span,
                Expr::Branch(Branch::If {
                    val,
                    t_block,
                    f_block,
                }),
            ),
        )),
        None => {
            let rhs_end = t_block.span.end;
            Ok((
                i,
                Node::new(
                    lhs.span + t_block.span,
                    Expr::Branch(Branch::If {
                        val,
                        t_block,
                        f_block: Node::new(rhs_end..rhs_end, ExprEnclosure(vec![])),
                    }),
                ),
            ))
        }
    }
}

pub fn match_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, lhs) = token_tag(Token::MATCH)(i)?;
    let (i, val) = or_test(i)?;
    let (i, _) = token_tag(Token::LBRACE)(i)?;

    let mut arms: Vec<Node<MatchArm>> = vec![];
    let mut i_ = i;
    let i = loop {
        let i = i_;
        let (i, tar) = opt(target_match)(i)?;
        let tar = match tar {
            Some(tar) => tar,
            None => break i,
        };
        let (i, _) = token_tag(Token::ARROW)(i)?;
        let (i, val) = opt(expr_enclosure)(i)?;
        let i = match val {
            Some(val) => {
                arms.push(Node::new(
                    tar.span + val.span,
                    MatchArm {
                        tar,
                        block: Node::new(val.span, MatchBlock(val.data.0)),
                    },
                ));
                i
            }
            None => {
                let (i, val) = expr(i)?;
                arms.push(Node::new(
                    tar.span + val.span,
                    MatchArm {
                        tar,
                        block: Node::new(val.span, MatchBlock(vec![val])),
                    },
                ));
                let (i, sep) = opt(token_tag(Token::COMMA))(i)?;
                match sep {
                    Some(_) => i,
                    None => break i,
                }
            }
        };
        i_ = i;
    };

    let (i, rhs) = token_tag(Token::RBRACE)(i)?;
    Ok((
        i,
        Node::new(
            lhs.span + rhs.span,
            Expr::Branch(Branch::Match { val, arms }),
        ),
    ))
}

pub fn match_arm<'input>(i: Input<'input>) -> KResult<'input, Node<MatchArm>> {
    let (i, tar) = target_match(i)?;
    let (i, _) = token_tag(Token::ARROW)(i)?;
    let (i, val) = expr_enclosure(i)?;
    Ok((
        i,
        Node::new(
            tar.span + val.span,
            MatchArm {
                tar,
                block: Node::new(val.span, MatchBlock(val.data.0)),
            },
        ),
    ))
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
