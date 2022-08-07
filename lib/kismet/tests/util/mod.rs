use std::ops::Range;

use kismet::{
    ast::*,
    parser::parse,
    types::{Integer, Node, Span},
};

#[allow(dead_code)]
pub fn assert_stmt(node: Node<Expr>, input: &str) {
    assert_eq!(
        Ok(Node::new(Span::from(input), ExprTop(vec![node]))),
        parse(input)
    )
}

#[allow(dead_code)]
pub fn new_arith(lhs: Node<Expr>, op: Node<OpArith>, rhs: Node<Expr>) -> Node<Expr> {
    Node::new(lhs.span + rhs.span, Expr::Op(Op::Arith(lhs, op, rhs)))
}

#[allow(dead_code)]
pub fn new_unary(op: Node<OpArith>, val: Node<Expr>) -> Node<Expr> {
    Node::new(op.span + val.span, Expr::Op(Op::Unary(op, val)))
}

#[allow(dead_code)]
pub fn new_op(range: Range<usize>, op: OpArith) -> Node<OpArith> {
    Node::new(range, op)
}

#[allow(dead_code)]
pub fn new_atom(range: Range<usize>, val: Atom) -> Node<Expr> {
    Node::new(range, Expr::Primary(Primary::Atom(val)))
}

#[allow(dead_code)]
pub fn new_integer(range: Range<usize>, val: Integer) -> Node<Expr> {
    new_atom(range, Atom::Integer(val))
}

#[allow(dead_code)]
pub fn new_string(range: Range<usize>, val: &str) -> Node<Expr> {
    new_atom(range, Atom::String(String::from(val)))
}

#[allow(dead_code)]
pub fn new_id(range: Range<usize>, val: &str) -> Node<Expr> {
    new_atom(range, Atom::Id(String::from(val)))
}
