use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, many1, separated_list0},
    sequence::preceded,
    Err,
};

use crate::{
    ast::{
        Branch, Expr, ExprEnclosure, Loop, LoopKind, MatchArm, Target, TargetKind, TargetListItem,
    },
    types::CommaList,
};
use crate::{
    ast::{Id, TargetExpr},
    types::{Node, ONode, Span},
};

use super::{
    or_test, target, target_match, token_tag, token_tag_id, ConvertKind, Error, ErrorKind, Input,
    KResult, Token,
};

pub fn expr_block0(i: Input) -> KResult<Option<Node<Vec<Node<Expr>>>>> {
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

pub fn expr_block1(i: Input) -> KResult<Node<Vec<Node<Expr>>>> {
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

pub fn expr_enclosure(i: Input) -> KResult<Node<ExprEnclosure>> {
    let (i, lhs) = token_tag(Token::LBRACE)(i)?;
    let (i, val) = expr_block0(i)?;
    let (i, rhs) = token_tag(Token::RBRACE)(i)?;
    match val {
        Some(val) => Ok((i, Node::new(lhs.span + rhs.span, ExprEnclosure(*val.data)))),
        None => Ok((i, Node::new(lhs.span + rhs.span, ExprEnclosure(vec![])))),
    }
}

pub fn assignment_expr(i: Input) -> KResult<Node<Expr>> {
    let (i, lhs) = expr(i)?;
    let (i, op) = opt(token_tag(Token::ASSIGNE))(i)?;
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

pub fn expr(i: Input) -> KResult<Node<Expr>> {
    branch_expr(i)
}

pub fn branch_expr(i: Input) -> KResult<Node<Expr>> {
    alt((
        if_expr,
        match_expr,
        map(loop_node, |x| Node::new(x.span, Expr::Loop(*x.data))),
        lambda_expr,
    ))(i)
}

pub fn if_expr(i: Input) -> KResult<Node<Expr>> {
    let (i, lhs) = token_tag(Token::IF)(i)?;
    let (i, val) = assignment_expr(i)?;
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

pub fn match_expr(i: Input) -> KResult<Node<Expr>> {
    let (i, lhs) = token_tag(Token::MATCH)(i)?;
    let (i, val) = assignment_expr(i)?;
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
                        block: Node::new(val.span, ExprEnclosure(val.data.0)),
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
                        block: Node::new(val.span, ExprEnclosure(vec![val])),
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

pub fn match_arm(i: Input) -> KResult<Node<MatchArm>> {
    let (i, tar) = target_match(i)?;
    let (i, _) = token_tag(Token::ARROW)(i)?;
    let (i, block) = expr_enclosure(i)?;
    Ok((i, Node::new(tar.span + block.span, MatchArm { tar, block })))
}

pub fn loop_node(i: Input) -> KResult<Node<Loop>> {
    let (i, id) = opt(loop_label)(i)?;
    let (i, val) = alt((for_expr, while_expr, loop_expr))(i)?;
    Ok((i, Node::new(Span::option(&id) + val.span, Loop { id, val })))
}

pub fn loop_label(i: Input) -> KResult<Node<Id>> {
    let (i, _) = token_tag(Token::COLON)(i)?;
    let (i, val) = token_tag_id(i)?;
    let (i, _) = token_tag(Token::COLON)(i)?;
    Ok((i, val))
}

pub fn for_expr(i: Input) -> KResult<Node<LoopKind>> {
    let (i, lhs) = token_tag(Token::FOR)(i)?;
    let (i, tar) = target(i)?;
    let (i, _) = token_tag(Token::IN)(i)?;
    let (i, val) = assignment_expr(i)?;
    let (i, block) = expr_enclosure(i)?;
    Ok((
        i,
        Node::new(lhs.span + block.span, LoopKind::For { tar, val, block }),
    ))
}

pub fn while_expr(i: Input) -> KResult<Node<LoopKind>> {
    let (i, lhs) = token_tag(Token::WHILE)(i)?;
    let (i, val) = assignment_expr(i)?;
    let (i, block) = expr_enclosure(i)?;
    Ok((
        i,
        Node::new(lhs.span + block.span, LoopKind::While { val, block }),
    ))
}

pub fn loop_expr(i: Input) -> KResult<Node<LoopKind>> {
    let (i, lhs) = token_tag(Token::LOOP)(i)?;
    let (i, block) = expr_enclosure(i)?;
    Ok((
        i,
        Node::new(lhs.span + block.span, LoopKind::Loop { block }),
    ))
}

pub fn lambda_expr(i: Input) -> KResult<Node<Expr>> {
    let (i, lhs) = match or_test(i) {
        Ok(x) => x,
        Err(Err::Failure(val)) => {
            let span = val.span;
            if let Error::Convert(i, ConvertKind::TargetKindExpr(lhs)) = *val.data {
                let (i, op) = opt(token_tag(Token::ARROW))(i)?;
                if op.is_none() {
                    return Err(Err::Failure(ONode::new(
                        span,
                        Error::Convert(i, ConvertKind::TargetKindExpr(lhs)),
                    )));
                }
                let args = match *lhs.data {
                    TargetKind::Id(x) => Node::new(
                        lhs.span,
                        CommaList(vec![Node::new(
                            lhs.span,
                            TargetListItem::Target(TargetExpr::Target(TargetKind::Id(x))),
                        )]),
                    ),
                    TargetKind::TargetTuple(x) => Node::new(lhs.span, CommaList(x)),
                    _ => {
                        return Err(Err::Failure(ONode::new(
                            span,
                            Error::Error(ErrorKind::Grammar),
                        )))
                    }
                };
                let (i, block) = expr_enclosure(i)?;
                return Ok((
                    i,
                    Node::new(lhs.span + block.span, Expr::Function { args, block }),
                ));
            } else {
                return Err(Err::Failure(val));
            }
        }
        Err(x) => return Err(x),
    };
    let lhs_span = lhs.span;
    let (i, op) = opt(token_tag(Token::ARROW))(i)?;
    let tar = match op {
        Some(_) => Node::<Target>::try_from(lhs)
            .map_err(|_| Err::Failure(ONode::new(lhs_span, Error::Error(ErrorKind::Grammar))))?,
        None => return Ok((i, lhs)),
    };
    let (i, block) = expr_enclosure(i)?;
    let args = match *tar.data {
        Target(TargetKind::Id(x)) => Node::new(
            lhs_span,
            CommaList(vec![Node::new(
                lhs_span,
                TargetListItem::Target(TargetExpr::Target(TargetKind::Id(x))),
            )]),
        ),
        Target(TargetKind::TargetTuple(x)) => Node::new(
            lhs_span,
            CommaList(
                x.into_iter()
                    .map(|x| Node::convert(TargetListItem::<TargetExpr>::convert, x))
                    .collect::<Vec<_>>(),
            ),
        ),
        _ => {
            return Err(Err::Failure(ONode::new(
                tar.span,
                Error::Error(ErrorKind::Grammar),
            )))
        }
    };
    Ok((
        i,
        Node::new(lhs_span + block.span, Expr::Function { args, block }),
    ))
}
