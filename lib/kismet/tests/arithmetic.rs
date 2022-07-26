use kismet::ast::OpArith;

mod util;
use util::{assert_stmt, new_arith, new_integer, new_unary};

#[test]
fn arithmetic() {
    assert_stmt(
        new_arith(new_integer(0..1, 2), OpArith::ADD, new_integer(2..3, 3)),
        r###"2+3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            OpArith::ADD,
            new_arith(new_integer(2..3, 3), OpArith::ADD, new_integer(4..5, 4)),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        new_arith(
            new_arith(new_integer(0..1, 2), OpArith::MUL, new_integer(2..3, 3)),
            OpArith::ADD,
            new_integer(4..5, 4),
        ),
        r###"2*3+4"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            OpArith::ADD,
            new_arith(new_integer(2..3, 3), OpArith::MUL, new_integer(4..5, 4)),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        new_arith(
            new_arith(new_integer(0..1, 2), OpArith::POW, new_integer(2..3, 5)),
            OpArith::ADD,
            new_arith(
                new_integer(4..5, 3),
                OpArith::MUL,
                new_arith(new_integer(6..7, 4), OpArith::POW, new_integer(8..9, 6)),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(
        new_unary(0..1, OpArith::ADD, new_integer(1..2, 3)),
        r###"+3"###,
    );
    assert_stmt(
        new_unary(0..1, OpArith::SUB, new_integer(1..2, 3)),
        r###"-3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            OpArith::ADD,
            new_unary(2..3, OpArith::ADD, new_integer(3..4, 3)),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            OpArith::SUB,
            new_unary(2..3, OpArith::SUB, new_integer(3..4, 3)),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            OpArith::MUL,
            new_unary(2..3, OpArith::ADD, new_integer(3..4, 3)),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        new_arith(
            new_integer(0..1, 2),
            OpArith::MUL,
            new_unary(2..3, OpArith::SUB, new_integer(3..4, 3)),
        ),
        r###"2*-3"###,
    );
}
