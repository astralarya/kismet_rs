use std::ops::Range;

use kismet_language::{
    ast::{Expr, Node},
    parse,
    types::{Integer, Span},
};

#[path = "./tests.rs"]
mod tests;
use tests::assert_stmt;

pub fn make_integer<'input>(range: Range<usize>, integer: Integer) -> Node<Expr<'input>> {
    Node::atom(Node::integer((Span(range), integer)))
}

pub fn make_string<'input>(range: Range<usize>, string: &'input str) -> Node<Expr<'input>> {
    Node::atom(Node::string((Span(range), String::from(string))))
}

pub fn make_id<'input>(range: Range<usize>, id: &'input str) -> Node<Expr<'input>> {
    Node::atom(Node::id((Span(range), id)))
}

#[test]
fn integer() {
    assert_stmt(make_integer(0..2, 42), r###"42"###);
    assert_stmt(make_integer(0..3, 42), r###"4_2"###);
    assert_stmt(make_integer(0..4, 42), r###"0x2A"###);
    assert_stmt(make_integer(0..5, 42), r###"0x2_A"###);
    assert_stmt(make_integer(0..4, 42), r###"0o52"###);
    assert_stmt(make_integer(0..5, 42), r###"0o5_2"###);
    assert_stmt(make_integer(0..8, 42), r###"0b101010"###);
    assert_stmt(make_integer(0..9, 42), r###"0b101_010"###);
    assert!(
        parse(&u128::MAX.to_string()).is_err(),
        "Parser should error on overflow"
    )
}

#[test]
fn string() {
    assert_stmt(make_string(0..7, "ababa"), r###""ababa""###);
    assert_stmt(make_string(0..9, "aba\"ba"), r###""aba\"ba""###);
    assert_stmt(make_string(0..11, "aba\\\"ba"), r###""aba\\\"ba""###);
    assert_stmt(
        make_string(0..16, r#"aba#aba"aba"#),
        r###"r#"aba#aba"aba"#"###,
    );
    assert_stmt(
        make_string(0..20, r##"aba"#aba#"aba"##),
        r###"r##"aba"#aba#"aba"##"###,
    );
}

#[test]
fn identifier() {
    assert_stmt(make_id(0..5, "ababa"), r###"ababa"###);
    assert_stmt(make_id(0..2, "da"), r###"da"###);
    assert!(
        parse(r###"d"###).is_err(),
        "Parser should not accept `d` as id"
    )
}
