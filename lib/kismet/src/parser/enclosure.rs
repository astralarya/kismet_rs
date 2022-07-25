use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::preceded,
};

use crate::{
    ast::{Atom, CompIter, ListItem, Target, TargetDictItem, TargetList},
    types::{Node, Span},
};

use super::{expr, id, token_action, token_if, token_tag, token_tag_id, Input, KResult, Token};

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

// pub fn comp_for<'input>(i: Input<'input>) -> KResult<'input, Node<CompIter>> {
//     let (i, lhs) = token_tag(Token::FOR)(i)?;
// }

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

pub fn list_item<'input>(i: Input<'input>) -> KResult<'input, Node<ListItem>> {
    let (i, lhs) = opt(token_tag(Token::SPREAD))(i)?;
    let (i, val) = expr(i)?;
    match lhs {
        Some(lhs) => Ok((i, Node::new(lhs.span + val.span, ListItem::Spread(val)))),
        None => Ok((i, Node::new(val.span, ListItem::Expr(*val.data)))),
    }
}
