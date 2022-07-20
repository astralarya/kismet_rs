use nom::{combinator::opt, sequence::tuple as nom_tuple};

use crate::ast::{Expr, Node, Primary};

use super::{atom, token_if, token_tag, KResult, Token};

pub fn expr<'input>(input: Node<&'input str>) -> KResult<Node<&'input str>, Node<Expr<'input>>> {
    a_expr(input)
}

pub fn a_expr<'input>(i: Node<&'input str>) -> KResult<Node<&'input str>, Node<Expr<'input>>> {
    let (i, lhs) = m_expr(i)?;
    let (i, rhs) = opt(nom_tuple((adds, a_expr)))(i)?;
    match rhs {
        Some((op, rhs)) => Ok((
            i,
            Node::new(lhs.span.clone() + rhs.span.clone(), Expr::Op(lhs, op, rhs)),
        )),
        None => Ok((i, lhs)),
    }
}

pub fn m_expr<'input>(i: Node<&'input str>) -> KResult<Node<&'input str>, Node<Expr<'input>>> {
    let (i, lhs) = p_expr(i)?;
    let (i, rhs) = opt(nom_tuple((muls, p_expr)))(i)?;
    match rhs {
        Some((op, rhs)) => Ok((
            i,
            Node::new(lhs.span.clone() + rhs.span.clone(), Expr::Op(lhs, op, rhs)),
        )),
        None => Ok((i, lhs)),
    }
}

pub fn p_expr<'input>(i: Node<&'input str>) -> KResult<Node<&'input str>, Node<Expr<'input>>> {
    let (i, lhs) = u_expr(i)?;
    let (i, rhs) = opt(nom_tuple((token_tag(Token::POW), u_expr)))(i)?;
    match rhs {
        Some((op, rhs)) => Ok((
            i,
            Node::new(lhs.span.clone() + rhs.span.clone(), Expr::Op(lhs, op, rhs)),
        )),
        None => Ok((i, lhs)),
    }
}

pub fn u_expr<'input>(i: Node<&'input str>) -> KResult<Node<&'input str>, Node<Expr<'input>>> {
    let (i, op) = opt(adds)(i)?;
    let (i, rhs) = coefficient(i)?;
    match op {
        Some(op) => Ok((
            i,
            Node::new(op.span.clone() + rhs.span.clone(), Expr::Unary(op, rhs)),
        )),
        None => Ok((i, rhs)),
    }
}

pub fn coefficient<'input>(i: Node<&'input str>) -> KResult<Node<&'input str>, Node<Expr<'input>>> {
    expr_node(i)
}

pub fn expr_node<'input>(i: Node<&'input str>) -> KResult<Node<&'input str>, Node<Expr<'input>>> {
    let (i, val) = atom(i)?;
    Ok((
        i,
        Node::new(val.span.clone(), Expr::Primary(Primary::Atom(*val.data))),
    ))
}

pub fn adds<'input>(i: Node<&'input str>) -> KResult<Node<&'input str>, Node<Token<'input>>> {
    token_if(|x| match *x.data {
        Token::ADD | Token::SUB => true,
        _ => false,
    })(i)
}

pub fn muls<'input>(i: Node<&'input str>) -> KResult<Node<&'input str>, Node<Token<'input>>> {
    token_if(|x| match *x.data {
        Token::MOD | Token::MUL | Token::DIV => true,
        _ => false,
    })(i)
}
