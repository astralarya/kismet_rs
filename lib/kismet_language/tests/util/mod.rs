use std::ops::Range;

use kismet_language::{
    ast::{Atom, Expr, Node},
    parse,
    token::{Token, TokenKind},
    types::{Integer, Span},
};

#[allow(dead_code)]
pub fn assert_stmt(node: Node<Expr>, input: &str) {
    assert_eq!(
        Ok(Node::new(Span(0..input.len()), Expr::Stmts(vec![node]))),
        parse(input)
    )
}

#[allow(dead_code)]
pub fn new_op<'input>(
    lhs: Node<Expr<'input>>,
    val: Token<'input>,
    rhs: Node<Expr<'input>>,
) -> Node<Expr<'input>> {
    Node {
        span: lhs.span.clone() + rhs.span.clone(),
        kind: Box::new(Expr::Op(lhs, val, rhs)),
    }
}

#[allow(dead_code)]
pub fn new_unary<'input>(lhs: Token<'input>, val: Node<Expr<'input>>) -> Node<Expr<'input>> {
    Node {
        span: lhs.span.clone() + val.span.clone(),
        kind: Box::new(Expr::Unary(lhs, val)),
    }
}

#[allow(dead_code)]
pub fn new_integer<'input>(range: Range<usize>, val: Integer) -> Node<Expr<'input>> {
    Node::new(Span(range), Expr::Atom(Atom::Integer(val)))
}

#[allow(dead_code)]
pub fn new_string<'input>(range: Range<usize>, val: &'input str) -> Node<Expr<'input>> {
    Node::new(Span(range), Expr::Atom(Atom::String(String::from(val))))
}

#[allow(dead_code)]
pub fn new_id<'input>(range: Range<usize>, val: &'input str) -> Node<Expr<'input>> {
    Node::new(Span(range), Expr::Atom(Atom::Id(val)))
}

#[allow(dead_code)]
pub fn new_token(range: Range<usize>, kind: TokenKind) -> Token {
    Token {
        span: Span(range),
        kind,
    }
}
