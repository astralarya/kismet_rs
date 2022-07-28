use std::ops::Range;

use kismet::{
    ast::*,
    parser::{parse, ParseNode},
    types::{Integer, Node, Span},
};

#[allow(dead_code)]
pub fn assert_stmt(node: ParseNode, input: &str) {
    assert_eq!(
        Ok(Node::new(Span::from(input), Expr::Stmts(vec![node]))),
        parse(input)
    )
}

#[allow(dead_code)]
pub fn new_arith<'input>(lhs: Node<Expr>, op: Node<OpArith>, rhs: Node<Expr>) -> Node<Expr> {
    Node::new(lhs.span + rhs.span, Expr::Arith(lhs, op, rhs))
}

#[allow(dead_code)]
pub fn new_unary<'input>(op: Node<OpArith>, val: Node<Expr>) -> Node<Expr> {
    Node::new(op.span + val.span, Expr::Unary(op, val))
}

#[allow(dead_code)]
pub fn new_op<'input>(range: Range<usize>, op: OpArith) -> Node<OpArith> {
    Node::new(range, op)
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
    new_atom(range, Atom::Id(String::from(val)))
}
