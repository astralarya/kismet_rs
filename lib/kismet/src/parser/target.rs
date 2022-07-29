use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::preceded,
};

use crate::{
    ast::{Target, TargetDictItem, TargetKind, TargetListItem},
    types::Node,
};

use super::{token_tag, token_tag_id, Input, KResult, Token};

pub fn target<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    alt((target_id, target_tuple, target_list, target_dict))(i)
}

pub fn target_id<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    map(token_tag_id, |x| {
        Node::new(x.span, Target(TargetKind::Id(x.data.to_string())))
    })(i)
}

pub fn target_tuple<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    let (i, lhs) = token_tag(Token::LPAREN)(i)?;
    let (i, val) = separated_list1(token_tag(Token::COMMA), target_list_item)(i)?;
    let (i, _) = opt(token_tag(Token::COMMA))(i)?;
    let (i, rhs) = token_tag(Token::RPAREN)(i)?;
    Ok((
        i,
        Node::new(lhs.span + rhs.span, Target(TargetKind::TargetTuple(val))),
    ))
}

pub fn target_list<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    let (i, lhs) = token_tag(Token::LBRACKET)(i)?;
    let (i, val) = separated_list1(token_tag(Token::COMMA), target_list_item)(i)?;
    let (i, _) = opt(token_tag(Token::COMMA))(i)?;
    let (i, rhs) = token_tag(Token::RBRACKET)(i)?;
    Ok((
        i,
        Node::new(lhs.span + rhs.span, Target(TargetKind::TargetList(val))),
    ))
}

pub fn target_dict<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    let (i, lhs) = token_tag(Token::LBRACE)(i)?;
    let (i, val) = separated_list1(token_tag(Token::COMMA), target_dict_item)(i)?;
    let (i, _) = opt(token_tag(Token::COMMA))(i)?;
    let (i, rhs) = token_tag(Token::RBRACE)(i)?;
    Ok((
        i,
        Node::new(lhs.span + rhs.span, Target(TargetKind::TargetDict(val))),
    ))
}

pub fn target_list_item<'input>(i: Input<'input>) -> KResult<'input, Node<TargetListItem<Target>>> {
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

pub fn target_dict_item<'input>(i: Input<'input>) -> KResult<'input, Node<TargetDictItem<Target>>> {
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
