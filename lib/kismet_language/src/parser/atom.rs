use crate::ast::{Atom, Expr, Node};

use super::{token_action, KResult, NumberKind, Token};

// pub fn atom<'input>(input: Node<&'input str>) -> KResult<Node<&'input str>, Node<Atom<'input>>> {
// }

pub fn numeric_literal<'input>(
    input: Node<&'input str>,
) -> KResult<Node<&'input str>, Node<Atom<'input>>> {
    token_action(|x| match *x.data {
        Token::Number(NumberKind::Integer(i)) => Some(Node::new(x.span, Atom::Integer(i))),
        _ => None,
    })(input)
}
