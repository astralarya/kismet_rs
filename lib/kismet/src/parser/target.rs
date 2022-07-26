use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::preceded,
};

use crate::{
    ast::{Target, TargetDictItem, TargetList},
    types::{Node, Span},
};

use super::{token_tag, token_tag_id, Input, KResult, Token};

pub fn target<'input>(i: Input<'input>) -> KResult<'input, Node<TargetList>> {
    let (i, val) = separated_list1(token_tag(Token::COMMA), target_item)(i)?;
    let (i, rhs) = opt(token_tag(Token::COMMA))(i)?;
    match (rhs, val.len()) {
        (None, 1) => Ok((
            i,
            Node::new(
                Span::from_iter(&val),
                TargetList::Target(*val[0].data.clone()),
            ),
        )),
        _ => Ok((i, Node::new(Span::from_iter(&val), TargetList::List(val)))),
    }
}

pub fn target_item<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    alt((target_id, target_tuple, target_list))(i)
}

pub fn target_id<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    map(token_tag_id, |x| {
        Node::new(x.span, Target::Id(x.data.to_string()))
    })(i)
}

pub fn target_tuple<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    let (i, lhs) = token_tag(Token::LPAREN)(i)?;
    let (i, val) = separated_list1(token_tag(Token::COMMA), target_item)(i)?;
    let (i, _) = opt(token_tag(Token::COMMA))(i)?;
    let (i, rhs) = token_tag(Token::RPAREN)(i)?;
    Ok((i, Node::new(lhs.span + rhs.span, Target::TargetList(val))))
}

pub fn target_list<'input>(i: Input<'input>) -> KResult<'input, Node<Target>> {
    let (i, lhs) = token_tag(Token::LBRACKET)(i)?;
    let (i, val) = separated_list1(token_tag(Token::COMMA), target_item)(i)?;
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

pub fn target_dict_item<'input>(i: Input<'input>) -> KResult<'input, Node<TargetDictItem>> {
    let (i, key) = token_tag_id(i)?;
    let (i, val) = opt(preceded(token_tag(Token::COLON), target_item))(i)?;
    match val {
        Some(val) => Ok((
            i,
            Node::new(key.span + val.span, TargetDictItem::Pair { key, val }),
        )),
        None => Ok((i, Node::new(key.span, TargetDictItem::Target(*key.data)))),
    }
}
