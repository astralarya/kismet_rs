use kismet_language::{ast::Node, Token};

pub fn op<'input>(l: Node<'input>, o: Token<'input>, r: Node<'input>) -> Node<'input> {
    Node::Op(Box::new(l), o, Box::new(r))
}

pub fn unary<'input>(o: Token<'input>, r: Node<'input>) -> Node<'input> {
    Node::Unary(o, Box::new(r))
}
