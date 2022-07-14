use kismet_language::{ast::Node, parse, types::Span};

pub fn assert_stmt(node: Node, str: &str) {
    assert_eq!(
        Ok(Node::stmts((Span(0..str.len()), vec![node]))),
        parse(str)
    )
}
