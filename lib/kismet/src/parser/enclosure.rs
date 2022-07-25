use nom::{branch::alt, combinator::opt};

use crate::{
    ast::{Atom, ListItem},
    types::Node,
};

use super::{expr, token_tag, Input, KResult, Token};

pub fn enclosure<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    alt((parentheses, list))(i)
}

pub fn parentheses<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    let (i, lhs) = token_tag(Token::LPAREN)(i)?;
    let (i, rhs) = opt(token_tag(Token::RPAREN))(i)?;
    match rhs {
        Some(rhs) => return Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(vec![])))),
        None => (),
    };
    let (i, val) = expr(i)?;
    let (i, rhs) = opt(token_tag(Token::RPAREN))(i)?;
    match rhs {
        Some(rhs) => return Ok((i, Node::new(lhs.span + rhs.span, Atom::Parentheses(val)))),
        None => (),
    }

    let (i, _) = token_tag(Token::COMMA)(i)?;
    let mut items = vec![val];
    let mut i = i;
    loop {
        let (_i, val) = opt(expr)(i)?;
        i = _i;
        match val {
            Some(val) => items.push(val),
            None => break,
        }
        let (_i, sep) = opt(token_tag(Token::COMMA))(i)?;
        i = _i;
        match sep {
            Some(_) => (),
            None => break,
        }
    }

    let (i, rhs) = token_tag(Token::RPAREN)(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(items))))
}

pub fn list<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    let (i, lhs) = token_tag(Token::LBRACKET)(i)?;
    let (i, rhs) = opt(token_tag(Token::RBRACKET))(i)?;
    match rhs {
        Some(rhs) => return Ok((i, Node::new(lhs.span + rhs.span, Atom::ListDisplay(vec![])))),
        None => (),
    };

    let (i, val) = list_item(i)?;
    // TODO comprehension

    let (i, _) = token_tag(Token::COMMA)(i)?;
    let mut items = vec![val];
    let mut i = i;
    loop {
        let (_i, val) = opt(list_item)(i)?;
        i = _i;
        match val {
            Some(val) => items.push(val),
            None => break,
        }
        let (_i, sep) = opt(token_tag(Token::COMMA))(i)?;
        i = _i;
        match sep {
            Some(_) => (),
            None => break,
        }
    }

    let (i, rhs) = token_tag(Token::RBRACKET)(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::ListDisplay(items))))
}

pub fn list_item<'input>(i: Input<'input>) -> KResult<'input, Node<ListItem>> {
    let (i, lhs) = opt(token_tag(Token::SPREAD))(i)?;
    let (i, val) = expr(i)?;
    match lhs {
        Some(lhs) => Ok((i, Node::new(lhs.span + val.span, ListItem::Spread(val)))),
        None => Ok((i, Node::new(val.span, ListItem::Expr(*val.data)))),
    }
}
