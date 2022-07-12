use kismet_language::{ast::Node, Token};

mod assert;
mod create;

use assert::assert_stmt;
use create::{op, unary};

#[test]
fn arithmetic() {
    assert_stmt(
        op(Node::Integer(2), Token::ADD, Node::Integer(3)),
        r###"2+3"###,
    );
    assert_stmt(
        op(
            op(Node::Integer(2), Token::ADD, Node::Integer(3)),
            Token::ADD,
            Node::Integer(4),
        ),
        r###"2+3+4"###,
    );
    assert_stmt(
        op(
            Node::Integer(2),
            Token::ADD,
            op(Node::Integer(3), Token::MUL, Node::Integer(4)),
        ),
        r###"2+3*4"###,
    );
    assert_stmt(
        op(
            op(Node::Integer(2), Token::POW, Node::Integer(5)),
            Token::ADD,
            op(
                Node::Integer(3),
                Token::MUL,
                op(Node::Integer(4), Token::POW, Node::Integer(6)),
            ),
        ),
        r###"2^5+3*4^6"###,
    );
    assert_stmt(unary(Token::ADD, Node::Integer(3)), r###"+3"###);
    assert_stmt(unary(Token::SUB, Node::Integer(3)), r###"-3"###);
    assert_stmt(
        op(
            Node::Integer(2),
            Token::ADD,
            unary(Token::ADD, Node::Integer(3)),
        ),
        r###"2++3"###,
    );
    assert_stmt(
        op(
            Node::Integer(2),
            Token::SUB,
            unary(Token::SUB, Node::Integer(3)),
        ),
        r###"2--3"###,
    );
    assert_stmt(
        op(
            Node::Integer(2),
            Token::MUL,
            unary(Token::ADD, Node::Integer(3)),
        ),
        r###"2*+3"###,
    );
    assert_stmt(
        op(
            Node::Integer(2),
            Token::MUL,
            unary(Token::SUB, Node::Integer(3)),
        ),
        r###"2*-3"###,
    );
}
