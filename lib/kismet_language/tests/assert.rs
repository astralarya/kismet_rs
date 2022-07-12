use kismet_language::{ast::Node, parse};

pub fn assert_stmt(node: Node, str: &str) {
    assert_eq!(Ok(Node::Stmts(vec![node])), parse(str))
}
