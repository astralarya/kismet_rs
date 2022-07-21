use std::ops::Range;

use kismet::{
    ast::*,
    parser::{parse, Token},
    types::{Integer, Span},
};

#[allow(dead_code)]
pub fn assert_stmt(node: Node<Expr>, input: &str) {
    assert_eq!(
        Ok(Node::new(Span::from(input), Expr::Stmts(vec![node]))),
        parse(input)
    )
}

#[allow(dead_code)]
pub fn new_op<'input>(
    lhs: Node<Expr<'input>>,
    val: Node<Token<'input>>,
    rhs: Node<Expr<'input>>,
) -> Node<Expr<'input>> {
    Node::new(
        lhs.span + rhs.span,
        Expr::Op(lhs, val, rhs),
    )
}

#[allow(dead_code)]
pub fn new_unary<'input>(lhs: Node<Token<'input>>, val: Node<Expr<'input>>) -> Node<Expr<'input>> {
    Node::new(lhs.span + val.span, Expr::Unary(lhs, val))
}

#[allow(dead_code)]
pub fn new_atom<'input>(range: Range<usize>, val: Atom<'input>) -> Node<Expr<'input>> {
    Node::new(range, Expr::Primary(Primary::Atom(val)))
}

#[allow(dead_code)]
pub fn new_integer<'input>(range: Range<usize>, val: Integer) -> Node<Expr<'input>> {
    new_atom(range, Atom::Integer(val))
}

#[allow(dead_code)]
pub fn new_string<'input>(range: Range<usize>, val: &'input str) -> Node<Expr<'input>> {
    new_atom(range, Atom::String(String::from(val)))
}

#[allow(dead_code)]
pub fn new_id<'input>(range: Range<usize>, val: &'input str) -> Node<Expr<'input>> {
    new_atom(range, Atom::Id(val))
}

#[allow(dead_code)]
pub fn new_token<'input>(range: Range<usize>, token: Token<'input>) -> Node<Token<'input>> {
    Node::new(range, token)
}
