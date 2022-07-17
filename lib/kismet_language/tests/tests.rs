use kismet_language::{
    ast::{Expr, Node},
    parse,
    types::Span,
};

pub fn assert_stmt(node: Node<Expr>, str: &str) {
    assert_eq!(
        Ok(Node::stmts((Span(0..str.len()), vec![node]))),
        parse(str)
    )
}
