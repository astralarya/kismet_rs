use std::ops::Range;

use kismet_language::{
    ast::{Expr, Node},
    parse,
    token::{Token, TokenKind},
    types::{Integer, Span},
};

pub fn assert_stmt(node: Node<Expr>, str: &str) {
    assert_eq!(
        Ok(Node::stmts((Span(0..str.len()), vec![node]))),
        parse(str)
    )
}

pub fn make_integer<'input>(range: Range<usize>, integer: Integer) -> Node<Expr<'input>> {
    Node::atom(Node::integer((Span(range), integer)))
}

#[allow(dead_code)]
pub fn make_string<'input>(range: Range<usize>, string: &'input str) -> Node<Expr<'input>> {
    Node::atom(Node::string((Span(range), String::from(string))))
}

#[allow(dead_code)]
pub fn make_id<'input>(range: Range<usize>, id: &'input str) -> Node<Expr<'input>> {
    Node::atom(Node::id((Span(range), id)))
}

#[allow(dead_code)]
pub fn make_token<'input>(range: Range<usize>, kind: TokenKind) -> Token {
    Token {
        span: Span(range),
        kind,
    }
}