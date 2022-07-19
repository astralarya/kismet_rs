use kismet_language::parser::Token;

mod util;
use util::{assert_stmt, new_integer, new_op, new_token, new_unary};

#[test]
fn arithmetic() {
    assert_stmt(
        new_op(
            new_integer(0..1, 2),
            new_token(1..2, Token::ADD),
            new_integer(2..3, 3),
        ),
        r###"2+3"###,
    );
    assert_stmt(
        new_op(
            new_op(
                new_integer(0..1, 2),
                new_token(1..2, Token::ADD),
                new_integer(2..3, 3),
            ),
            new_token(3..4, Token::ADD),
            new_integer(4..5, 4),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        new_op(
            new_integer(0..1, 2),
            new_token(1..2, Token::ADD),
            new_op(
                new_integer(2..3, 3),
                new_token(3..4, Token::MUL),
                new_integer(4..5, 4),
            ),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        new_op(
            new_op(
                new_integer(0..1, 2),
                new_token(1..2, Token::POW),
                new_integer(2..3, 5),
            ),
            new_token(3..4, Token::ADD),
            new_op(
                new_integer(4..5, 3),
                new_token(5..6, Token::MUL),
                new_op(
                    new_integer(6..7, 4),
                    new_token(7..8, Token::POW),
                    new_integer(8..9, 6),
                ),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(
        new_unary(new_token(0..1, Token::ADD), new_integer(1..2, 3)),
        r###"+3"###,
    );
    assert_stmt(
        new_unary(new_token(0..1, Token::SUB), new_integer(1..2, 3)),
        r###"-3"###,
    );
    assert_stmt(
        new_op(
            new_integer(0..1, 2),
            new_token(1..2, Token::ADD),
            new_unary(new_token(2..3, Token::ADD), new_integer(3..4, 3)),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        new_op(
            new_integer(0..1, 2),
            new_token(1..2, Token::SUB),
            new_unary(new_token(2..3, Token::SUB), new_integer(3..4, 3)),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        new_op(
            new_integer(0..1, 2),
            new_token(1..2, Token::MUL),
            new_unary(new_token(2..3, Token::ADD), new_integer(3..4, 3)),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        new_op(
            new_integer(0..1, 2),
            new_token(1..2, Token::MUL),
            new_unary(new_token(2..3, Token::SUB), new_integer(3..4, 3)),
        ),
        r###"2*-3"###,
    );
}
