use kismet_language::{ast::Node, token::Token};

mod assert;
use assert::assert_stmt;

#[test]
fn arithmetic() {
    assert_stmt(
        Node::op(
            Node::integer((0..1, 2)),
            Token::ADD(1..2),
            Node::integer((2..3, 3)),
        ),
        r###"2+3"###,
    );
    assert_stmt(
        Node::op(
            Node::op(
                Node::integer((0..1, 2)),
                Token::ADD(1..2),
                Node::integer((2..3, 3)),
            ),
            Token::ADD(3..4),
            Node::integer((4..5, 4)),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        Node::op(
            Node::integer((0..1, 2)),
            Token::ADD(1..2),
            Node::op(
                Node::integer((2..3, 3)),
                Token::MUL(3..4),
                Node::integer((4..5, 4)),
            ),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        Node::op(
            Node::op(
                Node::integer((0..1, 2)),
                Token::POW(1..2),
                Node::integer((2..3, 5)),
            ),
            Token::ADD(3..4),
            Node::op(
                Node::integer((4..5, 3)),
                Token::MUL(5..6),
                Node::op(
                    Node::integer((6..7, 4)),
                    Token::POW(7..8),
                    Node::integer((8..9, 6)),
                ),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(
        Node::unary(Token::ADD(0..1), Node::integer((1..2, 3))),
        r###"+3"###,
    );
    assert_stmt(
        Node::unary(Token::SUB(0..1), Node::integer((1..2, 3))),
        r###"-3"###,
    );
    assert_stmt(
        Node::op(
            Node::integer((0..1, 2)),
            Token::ADD(1..2),
            Node::unary(Token::ADD(2..3), Node::integer((3..4, 3))),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        Node::op(
            Node::integer((0..1, 2)),
            Token::SUB(1..2),
            Node::unary(Token::SUB(2..3), Node::integer((3..4, 3))),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        Node::op(
            Node::integer((0..1, 2)),
            Token::MUL(1..2),
            Node::unary(Token::ADD(2..3), Node::integer((3..4, 3))),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        Node::op(
            Node::integer((0..1, 2)),
            Token::MUL(1..2),
            Node::unary(Token::SUB(2..3), Node::integer((3..4, 3))),
        ),
        r###"2*-3"###,
    );
}
