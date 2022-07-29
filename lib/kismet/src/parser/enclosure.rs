use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
    Err,
};

use crate::{
    ast::{Atom, CompIter, DictItem, DictItemComp, ListItem},
    types::{Node, ONode},
};

use super::{
    expr, expr_list1, or_test, target, token_tag, token_tag_id, ErrorKind, Input, KResult, Token,
};

pub fn enclosure<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    alt((parens, brackets, brace))(i)
}

pub fn parens<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    let open = &token_tag(Token::LPAREN);
    let close = &token_tag(Token::RPAREN);
    let separator = &token_tag(Token::COMMA);

    let (i, lhs) = open(i)?;
    let (i, rhs) = opt(close)(i)?;
    match rhs {
        Some(rhs) => return Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(vec![])))),
        None => (),
    };
    let (i, val) = list_item(i)?;
    let (i, rhs) = opt(close)(i)?;
    match rhs {
        Some(rhs) => return Ok((i, Node::new(lhs.span + rhs.span, Atom::Paren(val)))),
        None => (),
    }

    let (i, comp_val) = opt(comp_for)(i)?;
    match comp_val {
        Some(comp_val) => {
            let (i, mut iter) = many0(alt((comp_for, comp_if)))(i)?;
            let (i, rhs) = close(i)?;
            iter.insert(0, comp_val);
            return Ok((
                i,
                Node::new(lhs.span + rhs.span, Atom::Generator { val, iter }),
            ));
        }
        None => (),
    }

    let (i, _) = separator(i)?;
    let (i, vals) = opt(terminated(
        separated_list1(separator, list_item),
        opt(separator),
    ))(i)?;
    let vals = match vals {
        Some(mut vals) => {
            vals.insert(0, val);
            vals
        }
        _ => vec![val],
    };

    let (i, rhs) = close(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(vals))))
}

pub fn brackets<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    let open = &token_tag(Token::LBRACKET);
    let close = &token_tag(Token::RBRACKET);
    let separator = &token_tag(Token::COMMA);

    let (i, lhs) = open(i)?;
    let (i, rhs) = opt(close)(i)?;
    match rhs {
        Some(rhs) => return Ok((i, Node::new(lhs.span + rhs.span, Atom::ListDisplay(vec![])))),
        None => (),
    };

    let (i, val) = list_item(i)?;
    let (i, comp_val) = opt(comp_for)(i)?;
    match comp_val {
        Some(comp_val) => {
            let (i, mut iter) = many0(alt((comp_for, comp_if)))(i)?;
            let (i, rhs) = close(i)?;
            iter.insert(0, comp_val);
            return Ok((
                i,
                Node::new(lhs.span + rhs.span, Atom::ListComprehension { val, iter }),
            ));
        }
        None => (),
    }

    let (i, vals) = opt(preceded(separator, separated_list1(separator, list_item)))(i)?;
    let (i, _) = opt(separator)(i)?;
    let vals = match vals {
        Some(mut vals) => {
            vals.insert(0, val);
            vals
        }
        _ => vec![val],
    };

    let (i, rhs) = close(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::ListDisplay(vals))))
}

pub fn list_item<'input>(i: Input<'input>) -> KResult<'input, Node<ListItem>> {
    let (i, lhs) = opt(token_tag(Token::SPREAD))(i)?;
    let (i, val) = expr(i)?;
    match lhs {
        Some(lhs) => Ok((i, Node::new(lhs.span + val.span, ListItem::Spread(val)))),
        None => Ok((i, Node::new(val.span, ListItem::Expr(*val.data)))),
    }
}

pub fn brace<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    let open = &token_tag(Token::LBRACE);
    let close = &token_tag(Token::RBRACE);
    let separator = &token_tag(Token::COMMA);

    let (i, lhs) = open(i)?;
    let (i, rhs) = opt(close)(i)?;
    match rhs {
        Some(rhs) => return Ok((i, Node::new(lhs.span + rhs.span, Atom::DictDisplay(vec![])))),
        None => (),
    };

    let (i, val) = opt(expr_list1)(i)?;
    match val {
        Some(val) => {
            let (i, rhs) = close(i)?;
            return Ok((i, Node::new(lhs.span + rhs.span, Atom::Block(*val.data))));
        }
        None => (),
    }

    let (i, val) = dict_item(i)?;
    let (i, comp_val) = opt(comp_for)(i)?;
    let val = Node::new(
        val.span,
        match (*val.data, comp_val) {
            (DictItem::DynKeyVal { key, val }, Some(comp_val)) => {
                let (i, mut iter) = many0(alt((comp_for, comp_if)))(i)?;
                let (i, rhs) = close(i)?;
                iter.insert(0, comp_val);
                return Ok((
                    i,
                    Node::new(
                        lhs.span + rhs.span,
                        Atom::DictComprehension {
                            val: Node::new(val.span, DictItemComp::DynKeyVal { key, val }),
                            iter,
                        },
                    ),
                ));
            }
            (DictItem::Spread(val), Some(comp_val)) => {
                let (i, mut iter) = many0(alt((comp_for, comp_if)))(i)?;
                iter.insert(0, comp_val);
                let (i, rhs) = close(i)?;
                return Ok((
                    i,
                    Node::new(
                        lhs.span + rhs.span,
                        Atom::DictComprehension {
                            val: Node::new(val.span, DictItemComp::Spread(val)),
                            iter,
                        },
                    ),
                ));
            }
            (_, Some(_)) => return Err(Err::Failure(ONode::new(val.span, ErrorKind::Grammar))),
            (val_data, None) => val_data,
        },
    );

    let (i, vals) = opt(preceded(separator, separated_list1(separator, dict_item)))(i)?;
    let (i, _) = opt(separator)(i)?;
    let vals = match vals {
        Some(mut vals) => {
            vals.insert(0, val);
            vals
        }
        _ => vec![val],
    };

    let (i, rhs) = close(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::DictDisplay(vals))))
}

pub fn dict_item<'input>(i: Input<'input>) -> KResult<'input, Node<DictItem>> {
    let spread = map(preceded(token_tag(Token::SPREAD), expr), |x| {
        Node::new(x.span, DictItem::Spread(x))
    });
    let dynkeyval = map(
        pair(
            delimited(token_tag(Token::LBRACKET), expr, token_tag(Token::RBRACKET)),
            preceded(token_tag(Token::COLON), expr),
        ),
        |(key, val)| Node::new(key.span + val.span, DictItem::DynKeyVal { key, val }),
    );
    let keyval = map(
        pair(token_tag_id, opt(preceded(token_tag(Token::COLON), expr))),
        |(key, val)| match val {
            Some(val) => Node::new(key.span + val.span, DictItem::KeyVal { key, val }),
            None => Node::new(key.span, DictItem::Shorthand(*key.data)),
        },
    );

    alt((spread, dynkeyval, keyval))(i)
}

pub fn comp_for<'input>(i: Input<'input>) -> KResult<'input, Node<CompIter>> {
    let (i, lhs) = token_tag(Token::FOR)(i)?;
    let (i, tar) = target(i)?;
    let (i, _) = token_tag(Token::IN)(i)?;
    let (i, val) = or_test(i)?;
    Ok((
        i,
        Node::new(lhs.span + val.span, CompIter::For { target: tar, val }),
    ))
}

pub fn comp_if<'input>(i: Input<'input>) -> KResult<'input, Node<CompIter>> {
    let (i, lhs) = token_tag(Token::IF)(i)?;
    let (i, val) = or_test(i)?;
    Ok((i, Node::new(lhs.span + val.span, CompIter::If(val))))
}
