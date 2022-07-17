use kismet_language::{ast::Node, token::Token, types::Span};

#[path = "./tests.rs"]
mod tests;
use tests::assert_stmt;

#[path = "./atoms.rs"]
mod atoms;
use atoms::make_integer;

#[test]
fn arithmetic() {
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            Token::ADD(Span(1..2)),
            make_integer(2..3, 3),
        ),
        r###"2+3"###,
    );
    assert_stmt(
        Node::op(
            Node::op(
                make_integer(0..1, 2),
                Token::ADD(Span(1..2)),
                make_integer(2..3, 3),
            ),
            Token::ADD(Span(3..4)),
            make_integer(4..5, 4),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            Token::ADD(Span(1..2)),
            Node::op(
                make_integer(2..3, 3),
                Token::MUL(Span(3..4)),
                make_integer(4..5, 4),
            ),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        Node::op(
            Node::op(
                make_integer(0..1, 2),
                Token::POW(Span(1..2)),
                make_integer(2..3, 5),
            ),
            Token::ADD(Span(3..4)),
            Node::op(
                make_integer(4..5, 3),
                Token::MUL(Span(5..6)),
                Node::op(
                    make_integer(6..7, 4),
                    Token::POW(Span(7..8)),
                    make_integer(8..9, 6),
                ),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(
        Node::unary(Token::ADD(Span(0..1)), make_integer(1..2, 3)),
        r###"+3"###,
    );
    assert_stmt(
        Node::unary(Token::SUB(Span(0..1)), make_integer(1..2, 3)),
        r###"-3"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            Token::ADD(Span(1..2)),
            Node::unary(Token::ADD(Span(2..3)), make_integer(3..4, 3)),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            Token::SUB(Span(1..2)),
            Node::unary(Token::SUB(Span(2..3)), make_integer(3..4, 3)),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            Token::MUL(Span(1..2)),
            Node::unary(Token::ADD(Span(2..3)), make_integer(3..4, 3)),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        Node::op(
            make_integer(0..1, 2),
            Token::MUL(Span(1..2)),
            Node::unary(Token::SUB(Span(2..3)), make_integer(3..4, 3)),
        ),
        r###"2*-3"###,
    );
}
