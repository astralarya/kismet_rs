use kismet_language::{ast::Node, Token};

mod assert;
use assert::assert_stmt;

#[test]
fn arithmetic() {
    assert_stmt(
        Node::to_op(Node::Integer(2), Token::ADD, Node::Integer(3)),
        r###"2+3"###,
    );
    assert_stmt(
        Node::to_op(
            Node::to_op(Node::Integer(2), Token::ADD, Node::Integer(3)),
            Token::ADD,
            Node::Integer(4),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        Node::to_op(
            Node::Integer(2),
            Token::ADD,
            Node::to_op(Node::Integer(3), Token::MUL, Node::Integer(4)),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        Node::to_op(
            Node::to_op(Node::Integer(2), Token::POW, Node::Integer(5)),
            Token::ADD,
            Node::to_op(
                Node::Integer(3),
                Token::MUL,
                Node::to_op(Node::Integer(4), Token::POW, Node::Integer(6)),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(Node::to_unary(Token::ADD, Node::Integer(3)), r###"+3"###);
    assert_stmt(Node::to_unary(Token::SUB, Node::Integer(3)), r###"-3"###);
    assert_stmt(
        Node::to_op(
            Node::Integer(2),
            Token::ADD,
            Node::to_unary(Token::ADD, Node::Integer(3)),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        Node::to_op(
            Node::Integer(2),
            Token::SUB,
            Node::to_unary(Token::SUB, Node::Integer(3)),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        Node::to_op(
            Node::Integer(2),
            Token::MUL,
            Node::to_unary(Token::ADD, Node::Integer(3)),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        Node::to_op(
            Node::Integer(2),
            Token::MUL,
            Node::to_unary(Token::SUB, Node::Integer(3)),
        ),
        r###"2*-3"###,
    );
}
