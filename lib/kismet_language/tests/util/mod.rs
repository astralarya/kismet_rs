use std::ops::Range;

use kismet_language::{
    ast::{Atom, Expr, Node},
    parse,
    token::{Token, TokenKind},
    types::{Integer, Span},
};

#[allow(dead_code)]
pub fn assert_stmt(node: Node<Expr>, str: &str) {
    assert_eq!(
        Ok(Node::new(Span(0..str.len()), Expr::Stmts(vec![node]))),
        parse(str)
    )
}

#[allow(dead_code)]
pub fn new_op<'input>(
    l: Node<Expr<'input>>,
    v: Token<'input>,
    r: Node<Expr<'input>>,
) -> Node<Expr<'input>> {
    Node {
        span: l.span.clone() + r.span.clone(),
        kind: Box::new(Expr::Op(l, v, r)),
    }
}

#[allow(dead_code)]
pub fn new_unary<'input>(l: Token<'input>, v: Node<Expr<'input>>) -> Node<Expr<'input>> {
    Node {
        span: l.span.clone() + v.span.clone(),
        kind: Box::new(Expr::Unary(l, v)),
    }
}

#[allow(dead_code)]
pub fn new_integer<'input>(range: Range<usize>, integer: Integer) -> Node<Expr<'input>> {
    Node::new(Span(range), Expr::Atom(Atom::Integer(integer)))
}

#[allow(dead_code)]
pub fn new_string<'input>(range: Range<usize>, string: &'input str) -> Node<Expr<'input>> {
    Node::new(Span(range), Expr::Atom(Atom::String(String::from(string))))
}

#[allow(dead_code)]
pub fn new_id<'input>(range: Range<usize>, id: &'input str) -> Node<Expr<'input>> {
    Node::new(Span(range), Expr::Atom(Atom::Id(id)))
}

#[allow(dead_code)]
pub fn new_token(range: Range<usize>, kind: TokenKind) -> Token {
    Token {
        span: Span(range),
        kind,
    }
}
