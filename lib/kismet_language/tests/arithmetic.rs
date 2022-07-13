use kismet_language::{ast::Node, Token};

mod assert;
use assert::assert_stmt;

#[test]
fn arithmetic() {
    assert_stmt(
        Node::op(Node::integer(2), Token::ADD, Node::integer(3)),
        r###"2+3"###,
    );
    assert_stmt(
        Node::op(
            Node::op(Node::integer(2), Token::ADD, Node::integer(3)),
            Token::ADD,
            Node::integer(4),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        Node::op(
            Node::integer(2),
            Token::ADD,
            Node::op(Node::integer(3), Token::MUL, Node::integer(4)),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        Node::op(
            Node::op(Node::integer(2), Token::POW, Node::integer(5)),
            Token::ADD,
            Node::op(
                Node::integer(3),
                Token::MUL,
                Node::op(Node::integer(4), Token::POW, Node::integer(6)),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(Node::unary(Token::ADD, Node::integer(3)), r###"+3"###);
    assert_stmt(Node::unary(Token::SUB, Node::integer(3)), r###"-3"###);
    assert_stmt(
        Node::op(
            Node::integer(2),
            Token::ADD,
            Node::unary(Token::ADD, Node::integer(3)),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        Node::op(
            Node::integer(2),
            Token::SUB,
            Node::unary(Token::SUB, Node::integer(3)),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        Node::op(
            Node::integer(2),
            Token::MUL,
            Node::unary(Token::ADD, Node::integer(3)),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        Node::op(
            Node::integer(2),
            Token::MUL,
            Node::unary(Token::SUB, Node::integer(3)),
        ),
        r###"2*-3"###,
    );
}
