use nom::sequence::preceded;
use nom::{combinator::opt, sequence::tuple, Err};

use crate::ast::{Expr, OpArith, OpEqs, OpRange, Primary, Range, Target};
use crate::types::{Node, Span};

use super::{numeric_literal, primary, token_action, token_tag, Error, Input, KResult, Token};

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
            Err(_) => Err(Err::Failure(Node::new(op.span, Error::Grammar))),
        },
        None => Ok((i, lhs)),
    }
}

pub fn conditional_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    lambda_expr(i)
}

pub fn lambda_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    or_test(i)
}

pub fn or_test<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, lhs) = and_test(i)?;
    let (i, rhs) = opt(preceded(token_tag(Token::OR), or_test))(i)?;
    match rhs {
        Some(rhs) => Ok((
            i,
            Node::new(lhs.span.clone() + rhs.span.clone(), Expr::Or(lhs, rhs)),
        )),
        None => Ok((i, lhs)),
    }
}

pub fn and_test<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, lhs) = not_test(i)?;
    let (i, rhs) = opt(preceded(token_tag(Token::AND), and_test))(i)?;
    match rhs {
        Some(rhs) => Ok((
            i,
            Node::new(lhs.span.clone() + rhs.span.clone(), Expr::And(lhs, rhs)),
        )),
        None => Ok((i, lhs)),
    }
}

pub fn not_test<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, op) = opt(token_tag(Token::NOT))(i)?;
    let (i, rhs) = c_expr(i)?;
    match op {
        Some(op) => Ok((
            i,
            Node::new(op.span.clone() + rhs.span.clone(), Expr::Not(rhs)),
        )),
        None => Ok((i, rhs)),
    }
}

pub fn c_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, lhs) = r_expr(i)?;
    let (i, val) = opt(tuple((eqs, r_expr)))(i)?;
    let (i, rhs) = opt(tuple((eqs, r_expr)))(i)?;
    match (val, rhs) {
        (Some((l_op, val)), Some((r_op, rhs))) => Ok((
            i,
            Node::new(
                lhs.span.clone() + rhs.span.clone(),
                Expr::CompareBound {
                    l_val: lhs,
                    l_op: *l_op.data,
                    val,
                    r_op: *r_op.data,
                    r_val: rhs,
                },
            ),
        )),
        (Some((op, rhs)), None) => Ok((
            i,
            Node::new(
                lhs.span.clone() + rhs.span.clone(),
                Expr::Compare(lhs, *op.data, rhs),
            ),
        )),
        (None, Some((op, rhs))) => Ok((
            i,
            Node::new(
                lhs.span.clone() + rhs.span.clone(),
                Expr::Compare(lhs, *op.data, rhs),
            ),
        )),
        (None, None) => Ok((i, lhs)),
    }
}

pub fn r_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, start) = opt(a_expr)(i)?;
    let (i, rhs) = opt(tuple((ranges, opt(a_expr))))(i)?;
    match (start, rhs) {
        (Some(start), Some((op, Some(end)))) => Ok((
            i,
            Node::new(
                start.span.clone() + end.span.clone(),
                Expr::Range(match *op.data {
                    OpRange::RANGE => Range::Range { start, end },
                    OpRange::RANGEI => Range::RangeI { start, end },
                }),
            ),
        )),
        (Some(start), Some((op, None))) => Ok((
            i,
            Node::new(
                start.span.clone() + op.span,
                Expr::Range(Range::RangeFrom { start }),
            ),
        )),
        (None, Some((op, Some(end)))) => Ok((
            i,
            Node::new(
                op.span.clone() + end.span.clone(),
                Expr::Range(match *op.data {
                    OpRange::RANGE => Range::RangeTo { end },
                    OpRange::RANGEI => Range::RangeToI { end },
                }),
            ),
        )),
        (None, Some((op, None))) => {
            Ok((i, Node::new(op.span.clone(), Expr::Range(Range::RangeFull))))
        }
        (Some(lhs), None) => Ok((i, lhs)),
        (None, None) => Err(Err::Error(Node::new(Span::from_iter(i), Error::Grammar))),
    }
}

pub fn a_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, lhs) = m_expr(i)?;
    let (i, rhs) = opt(tuple((adds, a_expr)))(i)?;
    match rhs {
        Some((op, rhs)) => Ok((
            i,
            Node::new(
                lhs.span.clone() + rhs.span.clone(),
                Expr::Arith(lhs, *op.data, rhs),
            ),
        )),
        None => Ok((i, lhs)),
    }
}

pub fn m_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, lhs) = p_expr(i)?;
    let (i, rhs) = opt(tuple((muls, m_expr)))(i)?;
    match rhs {
        Some((op, rhs)) => Ok((
            i,
            Node::new(
                lhs.span.clone() + rhs.span.clone(),
                Expr::Arith(lhs, *op.data, rhs),
            ),
        )),
        None => Ok((i, lhs)),
    }
}

pub fn p_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, lhs) = u_expr(i)?;
    let (i, rhs) = opt(tuple((pow, p_expr)))(i)?;
    match rhs {
        Some((op, rhs)) => Ok((
            i,
            Node::new(
                lhs.span.clone() + rhs.span.clone(),
                Expr::Arith(lhs, *op.data, rhs),
            ),
        )),
        None => Ok((i, lhs)),
    }
}

pub fn u_expr<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, op) = opt(adds)(i)?;
    let (i, rhs) = coefficient(i)?;
    match op {
        Some(op) => Ok((
            i,
            Node::new(
                op.span.clone() + rhs.span.clone(),
                Expr::Unary(*op.data, rhs),
            ),
        )),
        None => Ok((i, rhs)),
    }
}

pub fn coefficient<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, lhs) = opt(numeric_literal)(i)?;
    let (i, rhs) = opt(die)(i)?;
    match (lhs, rhs) {
        (Some(lhs), Some(rhs)) => Ok((
            i,
            Node::new(
                lhs.span.clone() + rhs.span.clone(),
                Expr::Coefficient(lhs, rhs),
            ),
        )),
        (Some(lhs), None) => Ok((
            i,
            Node::new(lhs.span.clone(), Expr::Primary(Primary::Atom(*lhs.data))),
        )),
        (None, Some(rhs)) => Ok((i, rhs)),
        (None, None) => Err(Err::Error(Node::new(Span::from_iter(i), Error::Grammar))),
    }
}

pub fn die<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, die_val) = opt(tuple((token_tag(Token::DIE), numeric_literal)))(i)?;
    match die_val {
        Some((op, rhs)) => Ok((i, Node::new(op.span + rhs.span, Expr::Die(rhs)))),
        None => expr_node(i),
    }
}

pub fn expr_node<'input>(i: Input<'input>) -> KResult<'input, Node<Expr>> {
    let (i, val) = primary(i)?;
    Ok((i, Node::new(val.span.clone(), Expr::Primary(*val.data))))
}

pub fn eqs<'input>(i: Input<'input>) -> KResult<'input, Node<OpEqs>> {
    token_action(|x| match *x.data {
        Token::EQ => Some(Node::new(x.span, OpEqs::EQ)),
        Token::NE => Some(Node::new(x.span, OpEqs::NE)),
        Token::LT => Some(Node::new(x.span, OpEqs::LT)),
        Token::LE => Some(Node::new(x.span, OpEqs::LE)),
        Token::GT => Some(Node::new(x.span, OpEqs::GT)),
        Token::GE => Some(Node::new(x.span, OpEqs::GE)),
        _ => None,
    })(i)
}

pub fn ranges<'input>(i: Input<'input>) -> KResult<'input, Node<OpRange>> {
    token_action(|x| match *x.data {
        Token::RANGE => Some(Node::new(x.span, OpRange::RANGE)),
        Token::RANGEI => Some(Node::new(x.span, OpRange::RANGEI)),
        _ => None,
    })(i)
}

pub fn adds<'input>(i: Input<'input>) -> KResult<'input, Node<OpArith>> {
    token_action(|x| match *x.data {
        Token::ADD => Some(Node::new(x.span, OpArith::ADD)),
        Token::SUB => Some(Node::new(x.span, OpArith::SUB)),
        _ => None,
    })(i)
}

pub fn muls<'input>(i: Input<'input>) -> KResult<'input, Node<OpArith>> {
    token_action(|x| match *x.data {
        Token::MOD => Some(Node::new(x.span, OpArith::MOD)),
        Token::MUL => Some(Node::new(x.span, OpArith::MUL)),
        Token::DIV => Some(Node::new(x.span, OpArith::DIV)),
        _ => None,
    })(i)
}

pub fn pow<'input>(i: Input<'input>) -> KResult<'input, Node<OpArith>> {
    token_action(|x| match *x.data {
        Token::POW => Some(Node::new(x.span, OpArith::POW)),
        _ => None,
    })(i)
}
