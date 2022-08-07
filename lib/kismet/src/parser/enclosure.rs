use nom::{
    branch::alt,
    combinator::opt,
    multi::{many0, separated_list1},
    sequence::preceded,
    Err,
};

use crate::{
    ast::{
        Atom, CompIter, DictItem, DictItemComp, Id, ListItem, TargetDictItem, TargetExpr,
        TargetKind, TargetListItem,
    },
    types::{Node, ONode, Span},
};

use super::{
    expr, expr_block0, or_test, target, target_dict_item, target_expr, target_list_item, token_tag,
    ConvertKind, Error, ErrorKind, Input, KResult, Token,
};

pub fn enclosure(i: Input) -> KResult<Node<Atom>> {
    alt((parens, brackets, brace))(i)
}

pub fn parens(i: Input) -> KResult<Node<Atom>> {
    let open = &token_tag(Token::LPAREN);
    let close = &token_tag(Token::RPAREN);
    let separator = &token_tag(Token::COMMA);

    let (i, lhs) = open(i)?;
    let (i, rhs) = opt(close)(i)?;
    if let Some(rhs) = rhs {
        return Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(vec![]))));
    };

    let vals = vec![];
    let (i, (_, val)) = target_tuple_result(lhs.span, vals, list_item(i))?;
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

    let vals = vec![val];
    let (mut _i, mut _vals) = (i, vals);
    let (i, vals) = loop {
        let (i, vals) = (_i, _vals);
        let (i, sep) = opt(separator)(i)?;
        if sep.is_none() {
            break (i, vals);
        }

        let (i, (mut vals, val)) = target_tuple_result(lhs.span, vals, opt(list_item)(i))?;
        match val {
            Some(val) => vals.push(val),
            None => break (i, vals),
        };
        (_i, _vals) = (i, vals);
    };

    let (i, rhs) = close(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::Tuple(vals))))
}

pub fn brackets(i: Input) -> KResult<Node<Atom>> {
    let open = &token_tag(Token::LBRACKET);
    let close = &token_tag(Token::RBRACKET);
    let separator = &token_tag(Token::COMMA);

    let (i, lhs) = open(i)?;
    let (i, rhs) = opt(close)(i)?;
    if let Some(rhs) = rhs {
        return Ok((i, Node::new(lhs.span + rhs.span, Atom::ListDisplay(vec![]))));
    };

    let (i, (_, val)) = target_list_result(lhs.span, vec![], list_item(i))?;

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

    let vals = vec![val];
    let (mut _i, mut _vals) = (i, vals);
    let (i, vals) = loop {
        let (i, vals) = (_i, _vals);
        let (i, sep) = opt(separator)(i)?;
        if sep.is_none() {
            break (i, vals);
        }

        let (i, (mut vals, val)) = target_list_result(lhs.span, vals, opt(list_item)(i))?;
        match val {
            Some(val) => vals.push(val),
            None => break (i, vals),
        };
        (_i, _vals) = (i, vals);
    };

    let (i, rhs) = close(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::ListDisplay(vals))))
}

pub fn list_item(i: Input) -> KResult<Node<ListItem>> {
    let (i, lhs) = opt(token_tag(Token::SPREAD))(i)?;

    let (i, val) = match expr(i) {
        Ok(x) => x,
        Err(Err::Failure(val)) => {
            let span = val.span;
            match *val.data {
                Error::Convert(i, ConvertKind::TargetKindExpr(val)) => {
                    let (i, ass) = opt(token_tag(Token::ASSIGN))(i)?;
                    if ass.is_some() {
                        let (i, rhs) = expr(i)?;
                        return Err(Err::Failure(ONode::new(
                            span,
                            Error::Convert(
                                i,
                                ConvertKind::TargetListItemExpr(Node::new(
                                    val.span + rhs.span,
                                    TargetListItem::Target(TargetExpr::TargetExpr(val, rhs)),
                                )),
                            ),
                        )));
                    }

                    return Err(Err::Failure(ONode::new(
                        span,
                        Error::Convert(
                            i,
                            ConvertKind::TargetListItemExpr(match lhs {
                                Some(lhs) => Node::new(
                                    lhs.span + val.span,
                                    TargetListItem::Spread(Node::<TargetExpr>::convert_from(val)),
                                ),
                                None => Node::convert(
                                    TargetListItem::Target,
                                    Node::<TargetExpr>::convert_from(val),
                                ),
                            }),
                        ),
                    )));
                }
                _ => return Err(Err::Failure(val)),
            }
        }
        Err(x) => return Err(x),
    };
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

pub fn target_tuple_result<T>(
    lhs_span: Span,
    vals: Vec<Node<ListItem>>,
    result: KResult<T>,
) -> KResult<(Vec<Node<ListItem>>, T)> {
    let kind = TargetKind::TargetTuple;
    let separator = token_tag(Token::COMMA);
    let close = token_tag(Token::RPAREN);

    match result {
        Ok((i, val)) => Ok((i, (vals, val))),
        Err(Err::Failure(val)) => {
            let span = val.span;
            match *val.data {
                Error::Convert(i, ConvertKind::TargetListItemExpr(val)) => {
                    let vals = vals
                        .iter()
                        .map(Node::<TargetListItem<TargetExpr>>::try_from)
                        .collect::<Result<Vec<_>, _>>();
                    let mut vals = match vals {
                        Ok(vals) => vals,
                        Err(_) => {
                            return Err(Err::Failure(ONode::new(
                                span,
                                Error::Error(ErrorKind::Grammar),
                            )))
                        }
                    };
                    vals.push(val);
                    let (i, more) = opt(preceded(
                        &separator,
                        separated_list1(&separator, target_list_item(&target_expr)),
                    ))(i)?;
                    let (i, _) = opt(&separator)(i)?;
                    if let Some(mut more) = more {
                        vals.append(&mut more)
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
                _ => Err(Err::Failure(val)),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn target_list_result<T>(
    lhs_span: Span,
    vals: Vec<Node<ListItem>>,
    result: KResult<T>,
) -> KResult<(Vec<Node<ListItem>>, T)> {
    let kind = TargetKind::TargetList;
    let separator = token_tag(Token::COMMA);
    let close = token_tag(Token::RBRACKET);

    match result {
        Ok((i, val)) => Ok((i, (vals, val))),
        Err(Err::Failure(val)) => {
            let span = val.span;
            match *val.data {
                Error::Convert(i, ConvertKind::TargetListItemExpr(val)) => {
                    let vals = vals
                        .iter()
                        .map(Node::<TargetListItem<TargetExpr>>::try_from)
                        .collect::<Result<Vec<_>, _>>();
                    let mut vals = match vals {
                        Ok(vals) => vals,
                        Err(_) => {
                            return Err(Err::Failure(ONode::new(
                                span,
                                Error::Error(ErrorKind::Grammar),
                            )))
                        }
                    };
                    vals.push(val);
                    let (i, more) = opt(preceded(
                        &separator,
                        separated_list1(&separator, target_list_item(&target_expr)),
                    ))(i)?;
                    let (i, _) = opt(&separator)(i)?;
                    if let Some(mut more) = more {
                        vals.append(&mut more)
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
                _ => Err(Err::Failure(val)),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn brace(i: Input) -> KResult<Node<Atom>> {
    let open = &token_tag(Token::LBRACE);
    let close = &token_tag(Token::RBRACE);
    let separator = &token_tag(Token::COMMA);

    let (i, lhs) = open(i)?;
    let (i, rhs) = opt(close)(i)?;
    if let Some(rhs) = rhs {
        return Ok((i, Node::new(lhs.span + rhs.span, Atom::DictDisplay(vec![]))));
    };

    let result = target_dict_result(lhs.span, vec![], dict_item(i));
    let (i, (_, val)) = match result {
        Ok(x) => x,
        Err(Err::Failure(val)) => match *val.data {
            Error::Convert(i, ConvertKind::ExprBlock(val)) => {
                return Ok((i, Node::convert(Atom::Block, val)));
            }
            _ => return Err(Err::Failure(val)),
        },
        Err(e) => return Err(e),
    };

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

    let vals = vec![val];
    let (mut _i, mut _vals) = (i, vals);
    let (i, vals) = loop {
        let (i, vals) = (_i, _vals);
        let (i, sep) = opt(separator)(i)?;
        if sep.is_none() {
            break (i, vals);
        }

        let result = target_dict_result(lhs.span, vals, opt(dict_item)(i));
        let (i, (mut vals, val)) = match result {
            Ok(x) => x,
            Err(Err::Failure(val)) => match *val.data {
                Error::Convert(_, ConvertKind::ExprBlock(_)) => {
                    return Err(Err::Failure(ONode::new(
                        val.span,
                        Error::Error(ErrorKind::Grammar),
                    )))
                }
                _ => return Err(Err::Failure(val)),
            },
            Err(e) => return Err(e),
        };
        match val {
            Some(val) => vals.push(val),
            None => break (i, vals),
        };
        (_i, _vals) = (i, vals);
    };

    let (i, rhs) = close(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Atom::DictDisplay(vals))))
}

pub fn dict_item(i: Input) -> KResult<Node<DictItem>> {
    let (i, lhs) = opt(token_tag(Token::LBRACKET))(i)?;
    if let Some(lhs) = lhs {
        let (i, key) = expr(i)?;
        let (i, _) = token_tag(Token::RBRACE)(i)?;
        let (i, _) = token_tag(Token::COLON)(i)?;
        let (i, val) = expr(i)?;
        return Ok((
            i,
            Node::new(lhs.span + val.span, DictItem::DynKeyVal { key, val }),
        ));
    }
    let (i, lhs) = opt(token_tag(Token::SPREAD))(i)?;

    let (i, key) = match expr(i) {
        Ok(x) => x,
        Err(Err::Failure(val)) => {
            let span = val.span;
            match *val.data {
                Error::Convert(i, ConvertKind::TargetKindExpr(val)) => {
                    let (i, ass) = opt(token_tag(Token::ASSIGN))(i)?;
                    if ass.is_some() {
                        let (i, rhs) = expr(i)?;
                        return Err(Err::Failure(ONode::new(
                            span,
                            Error::Convert(
                                i,
                                ConvertKind::TargetDictItemExpr(Node::new(
                                    val.span + rhs.span,
                                    TargetDictItem::Target(TargetExpr::TargetExpr(val, rhs)),
                                )),
                            ),
                        )));
                    }

                    return Err(Err::Failure(ONode::new(
                        span,
                        Error::Convert(
                            i,
                            ConvertKind::TargetDictItemExpr(match lhs {
                                Some(lhs) => Node::new(
                                    lhs.span + val.span,
                                    TargetDictItem::Spread(Node::<TargetExpr>::convert_from(val)),
                                ),
                                None => Node::convert(
                                    TargetDictItem::Target,
                                    Node::<TargetExpr>::convert_from(val),
                                ),
                            }),
                        ),
                    )));
                }
                _ => return Err(Err::Failure(val)),
            }
        }
        Err(x) => return Err(x),
    };
    let key_span = key.span;

    let (i, delim) = opt(token_tag(Token::DELIM))(i)?;
    match delim {
        Some(delim) => {
            let (i, vals) = expr_block0(i)?;
            let mut vals = match vals {
                Some(val) => *val.data,
                None => vec![],
            };
            let (i, rhs) = token_tag(Token::RBRACE)(i)?;
            vals.insert(0, key);
            return Err(Err::Failure(ONode::new(
                delim.span,
                Error::Convert(
                    i,
                    ConvertKind::ExprBlock(Node::new(key_span + rhs.span, vals)),
                ),
            )));
        }
        None => (),
    }
    let (i, val) = match lhs {
        Some(_) => (i, None),
        None => opt(preceded(token_tag(Token::COLON), expr))(i)?,
    };
    let (i, ass) = opt(preceded(token_tag(Token::ASSIGN), expr))(i)?;

    if let (Some(lhs), None) = (&lhs, &ass) {
        return Ok((i, Node::new(lhs.span + key.span, DictItem::Spread(key))));
    }
    match Node::<Id>::try_from(&key) {
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
                                    TargetExpr::TargetExpr(
                                        Node::<TargetKind<TargetExpr>>::convert_from(key),
                                        ass,
                                    ),
                                )),
                                (None, Some(val)) => match Node::<Id>::try_from(&val) {
                                    Ok(val) => TargetDictItem::KeyVal {
                                        key,
                                        val: Node::new(
                                            span,
                                            TargetExpr::TargetExpr(
                                                Node::<TargetKind<TargetExpr>>::convert_from(val),
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
                Some(val) => Ok((
                    i,
                    Node::new(key.span + val.span, DictItem::KeyVal { key, val }),
                )),
                None => Ok((i, Node::convert(DictItem::Shorthand, key))),
            },
        },
        Err(_) => Err(Err::Failure(ONode::new(
            key.span,
            Error::Error(ErrorKind::Grammar),
        ))),
    }
}

pub fn target_dict_result<T>(
    lhs_span: Span,
    vals: Vec<Node<DictItem>>,
    result: KResult<T>,
) -> KResult<(Vec<Node<DictItem>>, T)> {
    let separator = token_tag(Token::COMMA);
    let close = token_tag(Token::RBRACE);

    match result {
        Ok((i, val)) => Ok((i, (vals, val))),
        Err(Err::Failure(val)) => {
            let span = val.span;
            match *val.data {
                Error::Convert(i, ConvertKind::TargetDictItemExpr(val)) => {
                    let vals = vals
                        .iter()
                        .map(Node::<TargetDictItem<TargetExpr>>::try_from)
                        .collect::<Result<Vec<_>, _>>();
                    let mut vals = match vals {
                        Ok(vals) => vals,
                        Err(_) => {
                            return Err(Err::Failure(ONode::new(
                                span,
                                Error::Error(ErrorKind::Grammar),
                            )))
                        }
                    };
                    vals.push(val);
                    let (i, more) = opt(preceded(
                        &separator,
                        separated_list1(&separator, target_dict_item(&target_expr)),
                    ))(i)?;
                    let (i, _) = opt(&separator)(i)?;
                    if let Some(mut more) = more {
                        vals.append(&mut more)
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
                _ => Err(Err::Failure(val)),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn comp_for(i: Input) -> KResult<Node<CompIter>> {
    let (i, lhs) = token_tag(Token::FOR)(i)?;
    let (i, tar) = target(i)?;
    let (i, _) = token_tag(Token::IN)(i)?;
    let (i, val) = or_test(i)?;
    Ok((
        i,
        Node::new(lhs.span + val.span, CompIter::For { target: tar, val }),
    ))
}

pub fn comp_if(i: Input) -> KResult<Node<CompIter>> {
    let (i, lhs) = token_tag(Token::IF)(i)?;
    let (i, val) = or_test(i)?;
    Ok((i, Node::new(lhs.span + val.span, CompIter::If(val))))
}
