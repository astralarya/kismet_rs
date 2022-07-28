use kismet::ast::OpArith;

mod util;
use util::{assert_stmt, new_arith, new_integer, new_op, new_unary};

#[test]
fn arithmetic() {
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            new_op(1..2, OpArith::ADD),
            new_integer(2..3, 3),
        ),
        r###"2+3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            new_op(1..2, OpArith::ADD),
            new_arith(
                new_integer(2..3, 3),
                new_op(3..4, OpArith::ADD),
                new_integer(4..5, 4),
            ),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        new_arith(
            new_arith(
                new_integer(0..1, 2),
                new_op(1..2, OpArith::MUL),
                new_integer(2..3, 3),
            ),
            new_op(3..4, OpArith::ADD),
            new_integer(4..5, 4),
        ),
        r###"2*3+4"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            new_op(1..2, OpArith::ADD),
            new_arith(
                new_integer(2..3, 3),
                new_op(3..4, OpArith::MUL),
                new_integer(4..5, 4),
            ),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        new_arith(
            new_arith(
                new_integer(0..1, 2),
                new_op(1..2, OpArith::POW),
                new_integer(2..3, 5),
            ),
            new_op(3..4, OpArith::ADD),
            new_arith(
                new_integer(4..5, 3),
                new_op(5..6, OpArith::MUL),
                new_arith(
                    new_integer(6..7, 4),
                    new_op(7..8, OpArith::POW),
                    new_integer(8..9, 6),
                ),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(
        new_unary(new_op(0..1, OpArith::ADD), new_integer(1..2, 3)),
        r###"+3"###,
    );
    assert_stmt(
        new_unary(new_op(0..1, OpArith::SUB), new_integer(1..2, 3)),
        r###"-3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            new_op(1..2, OpArith::ADD),
            new_unary(new_op(2..3, OpArith::ADD), new_integer(3..4, 3)),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            new_op(1..2, OpArith::SUB),
            new_unary(new_op(2..3, OpArith::SUB), new_integer(3..4, 3)),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            new_op(1..2, OpArith::MUL),
            new_unary(new_op(2..3, OpArith::ADD), new_integer(3..4, 3)),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            new_op(1..2, OpArith::MUL),
            new_unary(new_op(2..3, OpArith::SUB), new_integer(3..4, 3)),
        ),
        r###"2*-3"###,
    );
}
