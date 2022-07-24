use std::ops::Range;

use kismet::{
    ast::*,
    parser::{parse, Token},
    types::{Integer, Node, Span},
};

#[allow(dead_code)]
pub fn assert_stmt(node: Node<Expr>, input: &str) {
    assert_eq!(
        Ok(Node::new(Span::from(input), Expr::Stmts(vec![node]))),
        parse(input)
    )
}

#[allow(dead_code)]
pub fn new_op<'input>(lhs: Node<Expr>, val: Node<Token>, rhs: Node<Expr>) -> Node<Expr> {
    Node::new(lhs.span + rhs.span, Expr::Op(lhs, val, rhs))
}

#[allow(dead_code)]
pub fn new_unary<'input>(lhs: Node<Token>, val: Node<Expr>) -> Node<Expr> {
    Node::new(lhs.span + val.span, Expr::Unary(lhs, val))
}

#[allow(dead_code)]
pub fn new_atom<'input>(range: Range<usize>, val: Atom) -> Node<Expr> {
    Node::new(range, Expr::Primary(Primary::Atom(val)))
}

#[allow(dead_code)]
pub fn new_integer<'input>(range: Range<usize>, val: Integer) -> Node<Expr> {
    new_atom(range, Atom::Integer(val))
}

#[allow(dead_code)]
pub fn new_string<'input>(range: Range<usize>, val: &'input str) -> Node<Expr> {
    new_atom(range, Atom::String(String::from(val)))
}

#[allow(dead_code)]
pub fn new_id<'input>(range: Range<usize>, val: &'input str) -> Node<Expr> {
    new_atom(range, Atom::Id(val))
}

#[allow(dead_code)]
pub fn new_token<'input>(range: Range<usize>, token: Token) -> Node<Token> {
    Node::new(range, token)
}
