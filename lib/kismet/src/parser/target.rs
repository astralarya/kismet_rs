use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::preceded,
};

use crate::{
    ast::{Match, Target, TargetDictItem, TargetExpr, TargetKind, TargetListItem},
    types::Node,
};

use super::{expr, literal, token_tag, token_tag_id, Input, KResult, Token};

pub fn target<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    map(target_kind(&target), |x| Node::new(x.span, Target(*x.data)))(i)
}

pub fn target_expr<'input>(i: Input<'input>) -> KResult<'input, Node<TargetExpr>> {
    let (i, tar) = target_kind(&target_expr)(i)?;
    let (i, val) = opt(preceded(token_tag(Token::ASSIGN), expr))(i)?;
    match val {
        Some(val) => Ok((
            i,
            Node::new(tar.span + val.span, TargetExpr::TargetExpr(tar, val)),
        )),
        None => Ok((i, Node::new(tar.span, TargetExpr::Target(*tar.data)))),
    }
}

pub fn target_match<'input>(i: Input<'input>) -> KResult<'input, Node<Match>> {
    alt((
        map(target_kind(&target_match), |x| {
            Node::new(x.span, Match::Target(*x.data))
        }),
        map(literal, |x| Node::new(x.span, Match::Literal(*x.data))),
    ))(i)
}

pub fn target_kind<'input, T>(
    target_atom: &'static impl Fn(Input<'input>) -> KResult<'input, Node<T>>,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<TargetKind<T>>>
where
    T: From<TargetKind<T>>,
{
    move |i| {
        alt((
            target_id,
            target_tuple(target_atom),
            target_list(target_atom),
            target_dict(target_atom),
        ))(i)
    }
}

pub fn target_id<'input, T>(i: Input<'input>) -> KResult<'input, Node<TargetKind<T>>> {
    map(token_tag_id, |x| {
        Node::new(x.span, TargetKind::Id(x.data.to_string()))
    })(i)
}

pub fn target_tuple<'input, T>(
    target_atom: &'static impl Fn(Input<'input>) -> KResult<'input, Node<T>>,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<TargetKind<T>>> {
    move |i| {
        let (i, lhs) = token_tag(Token::LPAREN)(i)?;
        let (i, val) = separated_list1(token_tag(Token::COMMA), target_list_item(target_atom))(i)?;
        let (i, _) = opt(token_tag(Token::COMMA))(i)?;
        let (i, rhs) = token_tag(Token::RPAREN)(i)?;
        Ok((
            i,
            Node::new(lhs.span + rhs.span, TargetKind::TargetTuple(val)),
        ))
    }
}

pub fn target_list<'input, T>(
    target_atom: &'static impl Fn(Input<'input>) -> KResult<'input, Node<T>>,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<TargetKind<T>>> {
    move |i| {
        let (i, lhs) = token_tag(Token::LBRACKET)(i)?;
        let (i, val) = separated_list1(token_tag(Token::COMMA), target_list_item(target_atom))(i)?;
        let (i, _) = opt(token_tag(Token::COMMA))(i)?;
        let (i, rhs) = token_tag(Token::RBRACKET)(i)?;
        Ok((
            i,
            Node::new(lhs.span + rhs.span, TargetKind::TargetList(val)),
        ))
    }
}

pub fn target_dict<'input, T>(
    target_atom: &'static impl Fn(Input<'input>) -> KResult<'input, Node<T>>,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<TargetKind<T>>>
where
    T: From<TargetKind<T>>,
{
    move |i| {
        let (i, lhs) = token_tag(Token::LBRACE)(i)?;
        let (i, val) = separated_list1(token_tag(Token::COMMA), target_dict_item(target_atom))(i)?;
        let (i, _) = opt(token_tag(Token::COMMA))(i)?;
        let (i, rhs) = token_tag(Token::RBRACE)(i)?;
        Ok((
            i,
            Node::new(lhs.span + rhs.span, TargetKind::TargetDict(val)),
        ))
    }
}

pub fn target_list_item<'input, T>(
    target_atom: &'static impl Fn(Input<'input>) -> KResult<'input, Node<T>>,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<TargetListItem<T>>> {
    move |i| {
        let (i, op) = opt(token_tag(Token::SPREAD))(i)?;
        let (i, val) = target_atom(i)?;
        match op {
            Some(op) => Ok((
                i,
                Node::new(op.span + val.span, TargetListItem::Spread(val)),
            )),
            None => Ok((i, Node::new(val.span, TargetListItem::Target(*val.data)))),
        }
    }
}

pub fn target_dict_item<'input, T>(
    target_atom: &'static impl Fn(Input<'input>) -> KResult<'input, Node<T>>,
) -> impl Fn(Input<'input>) -> KResult<'input, Node<TargetDictItem<T>>>
where
    T: From<TargetKind<T>>,
{
    move |i| {
        let (i, op) = opt(token_tag(Token::SPREAD))(i)?;
        match op {
            Some(op) => {
                let (i, val) = target_atom(i)?;
                return Ok((
                    i,
                    Node::new(op.span + val.span, TargetDictItem::Spread(val)),
                ));
            }
            None => (),
        }
        let (i, key) = token_tag_id(i)?;
        let (i, val) = opt(preceded(token_tag(Token::COLON), target_atom))(i)?;
        match val {
            Some(val) => Ok((
                i,
                Node::new(key.span + val.span, TargetDictItem::Pair { key, val }),
            )),
            None => Ok((
                i,
                Node::new(
                    key.span,
                    TargetDictItem::Target(T::from(TargetKind::Id(*key.data))),
                ),
            )),
        }
    }
}
