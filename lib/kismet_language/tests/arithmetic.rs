use kismet_language::{ast::Node, token::TokenKind};

#[path = "./tests.rs"]
mod tests;
use tests::*;

#[test]
fn arithmetic() {
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            make_token(1..2, TokenKind::ADD),
            make_integer(2..3, 3),
        ),
        r###"2+3"###,
    );
    assert_stmt(
        Node::op(
            Node::op(
                make_integer(0..1, 2),
                make_token(1..2, TokenKind::ADD),
                make_integer(2..3, 3),
            ),
            make_token(3..4, TokenKind::ADD),
            make_integer(4..5, 4),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            make_token(1..2, TokenKind::ADD),
            Node::op(
                make_integer(2..3, 3),
                make_token(3..4, TokenKind::MUL),
                make_integer(4..5, 4),
            ),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        Node::op(
            Node::op(
                make_integer(0..1, 2),
                make_token(1..2, TokenKind::POW),
                make_integer(2..3, 5),
            ),
            make_token(3..4, TokenKind::ADD),
            Node::op(
                make_integer(4..5, 3),
                make_token(5..6, TokenKind::MUL),
                Node::op(
                    make_integer(6..7, 4),
                    make_token(7..8, TokenKind::POW),
                    make_integer(8..9, 6),
                ),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(
        Node::unary(make_token(0..1, TokenKind::ADD), make_integer(1..2, 3)),
        r###"+3"###,
    );
    assert_stmt(
        Node::unary(make_token(0..1, TokenKind::SUB), make_integer(1..2, 3)),
        r###"-3"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            make_token(1..2, TokenKind::ADD),
            Node::unary(make_token(2..3, TokenKind::ADD), make_integer(3..4, 3)),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            make_token(1..2, TokenKind::SUB),
            Node::unary(make_token(2..3, TokenKind::SUB), make_integer(3..4, 3)),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            make_token(1..2, TokenKind::MUL),
            Node::unary(make_token(2..3, TokenKind::ADD), make_integer(3..4, 3)),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            make_token(1..2, TokenKind::MUL),
            Node::unary(make_token(2..3, TokenKind::SUB), make_integer(3..4, 3)),
        ),
        r###"2*-3"###,
    );
}
