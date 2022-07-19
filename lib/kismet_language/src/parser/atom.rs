use crate::ast::{Expr, Node};

use super::{token, KResult};

pub fn atom<'input>(input: Node<&'input str>) -> KResult<Node<&'input str>, Node<Expr<'input>>> {
    let (tail, head) = token(input)?;
    Ok((tail, Node::new(head.span, Expr::Token(*head.data))))
}
