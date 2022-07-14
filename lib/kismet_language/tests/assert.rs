use kismet_language::{ast::Node, parse};

pub fn assert_stmt(node: Node, str: &str) {
    assert_eq!(Ok(Node::stmts((0..str.len(), vec![node]))), parse(str))
}
