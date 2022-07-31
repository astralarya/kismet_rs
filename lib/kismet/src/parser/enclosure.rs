use nom::{
    branch::alt,
    combinator::opt,
    multi::{many0, separated_list1},
    sequence::preceded,
    Err,
};

use crate::{
    ast::{
        Atom, CompIter, DictItem, DictItemComp, ListItem, TargetDictItem, TargetExpr, TargetKind,
        TargetListItem,
    },
    types::{Node, ONode, Span},
};

use super::{
    expr, expr_block1, or_test, target, target_dict_item, target_expr, target_list_item, token_tag,
    ConvertKind, Error, ErrorKind, Input, KResult, Token,
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
    if let Some(rhs) = rhs {
        return Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(vec![]))));
    };

    let (i, val) = list_item_convert(TargetKind::TargetTuple, lhs.span)(i)?;
    let (i, rhs) = opt(close)(i)?;
    if let Some(rhs) = rhs {
        return Ok((i, Node::new(lhs.span + rhs.span, Atom::Paren(val))));
    }

    let (i, comp_val) = opt(comp_for)(i)?;
    if let Some(comp_val) = comp_val {
        let (i, mut iter) = many0(alt((comp_for, comp_if)))(i)?;
        let (i, rhs) = close(i)?;
        iter.insert(0, comp_val);
        return Ok((
            i,
            Node::new(lhs.span + rhs.span, Atom::Generator { val, iter }),
        ));
    }

    let (i, rhs) = opt(close)(i)?;
    if let Some(rhs) = rhs {
        return Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(vec![val]))));
    }

    let mut vals = vec![val];
    let mut _i = i;
    let i = loop {
        let i = _i;
        let (i, sep) = opt(separator)(i)?;
        if let None = sep {
            break i;
        }

        let (i, val) = opt(list_item_convert(TargetKind::TargetTuple, lhs.span))(i)?;
        match val {
            Some(val) => vals.push(val),
            None => break i,
        }
        _i = i;
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
    if let Some(rhs) = rhs {
        return Ok((i, Node::new(lhs.span + rhs.span, Atom::ListDisplay(vec![]))));
    };

    let (i, val) = list_item_convert(TargetKind::TargetList, lhs.span)(i)?;

    let (i, comp_val) = opt(comp_for)(i)?;
    if let Some(comp_val) = comp_val {
        let (i, mut iter) = many0(alt((comp_for, comp_if)))(i)?;
        let (i, rhs) = close(i)?;
        iter.insert(0, comp_val);
        return Ok((
            i,
            Node::new(lhs.span + rhs.span, Atom::ListComprehension { val, iter }),
        ));
    }

    let (i, rhs) = opt(close)(i)?;
    if let Some(rhs) = rhs {
        return Ok((
            i,
            Node::new(lhs.span + rhs.span, Atom::ListDisplay(vec![val])),
        ));
    }

    let mut vals = vec![val];
    let mut _i = i;
    let i = loop {
        let i = _i;
        let (i, sep) = opt(separator)(i)?;
        if let None = sep {
            break i;
        }

        let (i, val) = opt(list_item_convert(TargetKind::TargetList, lhs.span))(i)?;
        match val {
            Some(val) => vals.push(val),
            None => break i,
        }
        _i = i;
    };

    let (i, rhs) = close(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::ListDisplay(vals))))
}

pub fn list_item<'input>(i: Input<'input>) -> KResult<'input, Node<ListItem>> {
    let (i, lhs) = opt(token_tag(Token::SPREAD))(i)?;
    let (i, val) = expr(i)?;
    let val_span = val.span;

    let (i, ass) = opt(token_tag(Token::ASSIGN))(i)?;
    if let Some(ass) = ass {
        let (i, rhs) = expr(i)?;
        match Node::<TargetKind<TargetExpr>>::try_from(val) {
            Ok(tar) => {
                return Err(Err::Failure(ONode::new(
                    ass.span,
                    Error::Convert(
                        i,
                        ConvertKind::TargetListItemExpr(Node::new(
                            val_span + rhs.span,
                            TargetListItem::Target(TargetExpr::TargetExpr(tar, rhs)),
                        )),
                    ),
                )));
            }
            Err(_) => {
                return Err(Err::Failure(ONode::new(
                    ass.span,
                    Error::Error(ErrorKind::Grammar),
                )))
            }
        }
    }

    match lhs {
        Some(lhs) => Ok((i, Node::new(lhs.span + val.span, ListItem::Spread(val)))),
        None => Ok((i, Node::convert(ListItem::Expr, val))),
    }
}

pub fn list_item_convert<'input>(
    kind: impl Fn(Vec<Node<TargetListItem<TargetExpr>>>) -> TargetKind<TargetExpr>,
    lhs_span: Span,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<ListItem>> {
    let separator = token_tag(Token::COMMA);
    let close = token_tag(Token::RBRACKET);

    move |i| match list_item(i) {
        Ok((i, val)) => Ok((i, val)),
        Err(Err::Failure(val)) => {
            let span = val.span;
            match *val.data {
                Error::Convert(i, ConvertKind::TargetListItemExpr(val)) => {
                    let (i, vals) = opt(preceded(
                        &separator,
                        separated_list1(&separator, target_list_item(&target_expr)),
                    ))(i)?;
                    let (i, _) = opt(&separator)(i)?;
                    let vals = match vals {
                        Some(mut vals) => {
                            vals.insert(0, val);
                            vals
                        }
                        _ => vec![val],
                    };
                    let (i, rhs) = close(i)?;

                    return Err(Err::Failure(ONode::new(
                        span,
                        Error::Convert(
                            i,
                            ConvertKind::TargetKindExpr(Node::new(lhs_span + rhs.span, kind(vals))),
                        ),
                    )));
                }
                _ => return Err(Err::Failure(val)),
            }
        }
        Err(e) => return Err(e),
    }
}

pub fn brace<'input>(i: Input<'input>) -> KResult<'input, Node<Atom>> {
    let open = &token_tag(Token::LBRACE);
    let close = &token_tag(Token::RBRACE);
    let separator = &token_tag(Token::COMMA);

    let (i, lhs) = open(i)?;
    let (i, rhs) = opt(close)(i)?;
    if let Some(rhs) = rhs {
        return Ok((i, Node::new(lhs.span + rhs.span, Atom::DictDisplay(vec![]))));
    };

    let (i, val) = opt(expr_block1)(i)?;
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
            (_, Some(_)) => {
                return Err(Err::Failure(ONode::new(
                    val.span,
                    Error::Error(ErrorKind::Grammar),
                )))
            }
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
    let (i, lhs) = opt(token_tag(Token::LBRACKET))(i)?;
    if let Some(lhs) = lhs {
        let (i, key) = expr(i)?;
        let (i, _) = token_tag(Token::RBRACKET)(i)?;
        let (i, _) = token_tag(Token::COLON)(i)?;
        let (i, val) = expr(i)?;
        return Ok((
            i,
            Node::new(lhs.span + val.span, DictItem::DynKeyVal { key, val }),
        ));
    }
    let (i, lhs) = opt(token_tag(Token::SPREAD))(i)?;
    let (i, key) = expr(i)?;
    let (i, val) = match lhs {
        Some(_) => (i, None),
        None => opt(preceded(token_tag(Token::COLON), expr))(i)?,
    };
    let (i, ass) = opt(preceded(token_tag(Token::ASSIGN), expr))(i)?;

    if let (Some(lhs), None) = (&lhs, &ass) {
        return Ok((i, Node::new(lhs.span + key.span, DictItem::Spread(key))));
    }
    match Node::<String>::try_from(&key) {
        Ok(key) => match ass {
            Some(ass) => {
                let span = Span::option_ref(&lhs) + key.span + ass.span;
                return Err(Err::Failure(ONode::new(
                    ass.span,
                    Error::Convert(
                        i,
                        ConvertKind::TargetDictItemExpr(Node::new(
                            span,
                            match (lhs, val) {
                                (None, None) => TargetDictItem::Target(TargetExpr::TargetExpr(
                                    Node::convert(TargetKind::Id, key),
                                    ass,
                                )),
                                (Some(_), None) => TargetDictItem::Spread(Node::new(
                                    span,
                                    TargetExpr::TargetExpr(Node::convert(TargetKind::Id, key), ass),
                                )),
                                (None, Some(val)) => match Node::<String>::try_from(&val) {
                                    Ok(val) => TargetDictItem::Pair {
                                        key,
                                        val: Node::new(
                                            span,
                                            TargetExpr::TargetExpr(
                                                Node::convert(TargetKind::Id, val),
                                                ass,
                                            ),
                                        ),
                                    },
                                    Err(_) => {
                                        return Err(Err::Failure(ONode::new(
                                            key.span,
                                            Error::Error(ErrorKind::Grammar),
                                        )))
                                    }
                                },
                                (Some(lhs), Some(_)) => {
                                    return Err(Err::Failure(ONode::new(
                                        lhs.span,
                                        Error::Error(ErrorKind::Grammar),
                                    )))
                                }
                            },
                        )),
                    ),
                )));
            }
            None => match val {
                Some(val) => {
                    return Ok((
                        i,
                        Node::new(key.span + val.span, DictItem::KeyVal { key, val }),
                    ))
                }
                None => return Ok((i, Node::convert(DictItem::Shorthand, key))),
            },
        },
        Err(_) => {
            return Err(Err::Failure(ONode::new(
                key.span,
                Error::Error(ErrorKind::Grammar),
            )))
        }
    }
}

pub fn dict_item_convert<'input>(
    lhs_span: Span,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<DictItem>> {
    let separator = token_tag(Token::COMMA);
    let close = token_tag(Token::RBRACKET);

    move |i| match dict_item(i) {
        Ok((i, val)) => Ok((i, val)),
        Err(Err::Failure(val)) => {
            let span = val.span;
            match *val.data {
                Error::Convert(i, ConvertKind::TargetDictItemExpr(val)) => {
                    let (i, vals) = opt(preceded(
                        &separator,
                        separated_list1(&separator, target_dict_item(&target_expr)),
                    ))(i)?;
                    let (i, _) = opt(&separator)(i)?;
                    let vals = match vals {
                        Some(mut vals) => {
                            vals.insert(0, val);
                            vals
                        }
                        _ => vec![val],
                    };
                    let (i, rhs) = close(i)?;

                    return Err(Err::Failure(ONode::new(
                        span,
                        Error::Convert(
                            i,
                            ConvertKind::TargetKindExpr(Node::new(
                                lhs_span + rhs.span,
                                TargetKind::TargetDict(vals),
                            )),
                        ),
                    )));
                }
                _ => return Err(Err::Failure(val)),
            }
        }
        Err(e) => return Err(e),
    }
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
