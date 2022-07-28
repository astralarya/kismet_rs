use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::preceded,
};

use crate::{
    ast::{Atom, DictItem, Expr, ListItem, Primary, Target, TargetDictItem, TargetListItem},
    types::Node,
};

use super::{token_tag, token_tag_id, Input, KResult, Token};

pub fn target<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    alt((target_id, target_tuple, target_list, target_dict))(i)
}

pub fn target_id<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    map(token_tag_id, |x| {
        Node::new(x.span, Target::Id(x.data.to_string()))
    })(i)
}

pub fn target_tuple<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    let (i, lhs) = token_tag(Token::LPAREN)(i)?;
    let (i, val) = separated_list1(token_tag(Token::COMMA), target_list_item)(i)?;
    let (i, _) = opt(token_tag(Token::COMMA))(i)?;
    let (i, rhs) = token_tag(Token::RPAREN)(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Target::TargetTuple(val))))
}

pub fn target_list<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    let (i, lhs) = token_tag(Token::LBRACKET)(i)?;
    let (i, val) = separated_list1(token_tag(Token::COMMA), target_list_item)(i)?;
    let (i, _) = opt(token_tag(Token::COMMA))(i)?;
    let (i, rhs) = token_tag(Token::RBRACKET)(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Target::TargetList(val))))
}

pub fn target_dict<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    let (i, lhs) = token_tag(Token::LBRACE)(i)?;
    let (i, val) = separated_list1(token_tag(Token::COMMA), target_dict_item)(i)?;
    let (i, _) = opt(token_tag(Token::COMMA))(i)?;
    let (i, rhs) = token_tag(Token::RBRACE)(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Target::TargetDict(val))))
}

pub fn target_list_item<'input>(i: Input<'input>) -> KResult<'input, Node<TargetListItem>> {
    let (i, op) = opt(token_tag(Token::SPREAD))(i)?;
    let (i, val) = target(i)?;
    match op {
        Some(op) => Ok((
            i,
            Node::new(op.span + val.span, TargetListItem::Spread(val)),
        )),
        None => Ok((i, Node::new(val.span, TargetListItem::Target(*val.data)))),
    }
}

pub fn target_dict_item<'input>(i: Input<'input>) -> KResult<'input, Node<TargetDictItem>> {
    let (i, op) = opt(token_tag(Token::SPREAD))(i)?;
    match op {
        Some(op) => {
            let (i, val) = target(i)?;
            return Ok((
                i,
                Node::new(op.span + val.span, TargetDictItem::Spread(val)),
            ));
        }
        None => (),
    }
    let (i, key) = token_tag_id(i)?;
    let (i, val) = opt(preceded(token_tag(Token::COLON), target))(i)?;
    match val {
        Some(val) => Ok((
            i,
            Node::new(key.span + val.span, TargetDictItem::Pair { key, val }),
        )),
        None => Ok((i, Node::new(key.span, TargetDictItem::Target(*key.data)))),
    }
}

pub fn expr_as_target(val: &Node<Expr>) -> Option<Node<Target>> {
    match &*val.data {
        Expr::Primary(Primary::Atom(Atom::Id(x))) => {
            Some(Node::new(val.span, Target::Id(x.clone())))
        }
        Expr::Primary(Primary::Atom(Atom::Tuple(x))) => {
            let y = x
                .iter()
                .filter_map(|x| match &*x.data {
                    ListItem::Expr(y) => match expr_as_target(&Node::new(x.span, y.clone())) {
                        Some(x) => Some(Node::new(x.span, TargetListItem::Target(*x.data))),
                        None => None,
                    },
                    ListItem::Spread(x) => match expr_as_target(x) {
                        Some(x) => Some(Node::new(x.span, TargetListItem::Spread(x))),
                        None => None,
                    },
                })
                .collect::<Vec<_>>();
            if x.len() != y.len() {
                return None;
            }
            Some(Node::new(val.span, Target::TargetTuple(y)))
        }
        Expr::Primary(Primary::Atom(Atom::ListDisplay(x))) => {
            let y = x
                .iter()
                .filter_map(|x| match &*x.data {
                    ListItem::Expr(y) => match expr_as_target(&Node::new(x.span, y.clone())) {
                        Some(x) => Some(Node::new(x.span, TargetListItem::Target(*x.data))),
                        None => None,
                    },
                    ListItem::Spread(x) => match expr_as_target(&x) {
                        Some(x) => Some(Node::new(x.span, TargetListItem::Spread(x))),
                        None => None,
                    },
                })
                .collect::<Vec<_>>();
            if x.len() != y.len() {
                return None;
            }
            Some(Node::new(val.span, Target::TargetList(y)))
        }
        Expr::Primary(Primary::Atom(Atom::DictDisplay(x))) => {
            let y = x
                .iter()
                .filter_map(|x| match &*x.data {
                    DictItem::Shorthand(y) => {
                        Some(Node::new(x.span, TargetDictItem::Target(y.clone())))
                    }
                    DictItem::Spread(x) => match expr_as_target(&x) {
                        Some(x) => Some(Node::new(x.span, TargetDictItem::Spread(x))),
                        None => None,
                    },
                    DictItem::KeyVal { key, val } => match expr_as_target(&val) {
                        Some(val) => Some(Node::new(
                            x.span,
                            TargetDictItem::Pair {
                                key: key.clone(),
                                val,
                            },
                        )),
                        None => None,
                    },
                    _ => None,
                })
                .collect::<Vec<_>>();
            if x.len() != y.len() {
                return None;
            }
            Some(Node::new(val.span, Target::TargetDict(y)))
        }
        _ => None,
    }
}
