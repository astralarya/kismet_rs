use kismet_language::{ast::Node, token::TokenKind};

mod util;
use util::{assert_stmt, new_integer, new_token};

#[test]
fn arithmetic() {
    assert_stmt(
        Node::op(
            new_integer(0..1, 2),
            new_token(1..2, TokenKind::ADD),
            new_integer(2..3, 3),
        ),
        r###"2+3"###,
    );
    assert_stmt(
        Node::op(
            Node::op(
                new_integer(0..1, 2),
                new_token(1..2, TokenKind::ADD),
                new_integer(2..3, 3),
            ),
            new_token(3..4, TokenKind::ADD),
            new_integer(4..5, 4),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        Node::op(
            new_integer(0..1, 2),
            new_token(1..2, TokenKind::ADD),
            Node::op(
                new_integer(2..3, 3),
                new_token(3..4, TokenKind::MUL),
                new_integer(4..5, 4),
            ),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        Node::op(
            Node::op(
                new_integer(0..1, 2),
                new_token(1..2, TokenKind::POW),
                new_integer(2..3, 5),
            ),
            new_token(3..4, TokenKind::ADD),
            Node::op(
                new_integer(4..5, 3),
                new_token(5..6, TokenKind::MUL),
                Node::op(
                    new_integer(6..7, 4),
                    new_token(7..8, TokenKind::POW),
                    new_integer(8..9, 6),
                ),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(
        Node::unary(new_token(0..1, TokenKind::ADD), new_integer(1..2, 3)),
        r###"+3"###,
    );
    assert_stmt(
        Node::unary(new_token(0..1, TokenKind::SUB), new_integer(1..2, 3)),
        r###"-3"###,
    );
    assert_stmt(
        Node::op(
            new_integer(0..1, 2),
            new_token(1..2, TokenKind::ADD),
            Node::unary(new_token(2..3, TokenKind::ADD), new_integer(3..4, 3)),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        Node::op(
            new_integer(0..1, 2),
            new_token(1..2, TokenKind::SUB),
            Node::unary(new_token(2..3, TokenKind::SUB), new_integer(3..4, 3)),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        Node::op(
            new_integer(0..1, 2),
            new_token(1..2, TokenKind::MUL),
            Node::unary(new_token(2..3, TokenKind::ADD), new_integer(3..4, 3)),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        Node::op(
            new_integer(0..1, 2),
            new_token(1..2, TokenKind::MUL),
            Node::unary(new_token(2..3, TokenKind::SUB), new_integer(3..4, 3)),
        ),
        r###"2*-3"###,
    );
}
