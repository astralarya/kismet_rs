use kismet_language::{ast::Node, parse, types::Span};

mod assert;
use assert::assert_stmt;

#[test]
fn integer() {
    assert_stmt(Node::integer((Span(0..2), 42)), r###"42"###);
    assert_stmt(Node::integer((Span(0..3), 42)), r###"4_2"###);
    assert_stmt(Node::integer((Span(0..4), 42)), r###"0x2A"###);
    assert_stmt(Node::integer((Span(0..5), 42)), r###"0x2_A"###);
    assert_stmt(Node::integer((Span(0..4), 42)), r###"0o52"###);
    assert_stmt(Node::integer((Span(0..5), 42)), r###"0o5_2"###);
    assert_stmt(Node::integer((Span(0..8), 42)), r###"0b101010"###);
    assert_stmt(Node::integer((Span(0..9), 42)), r###"0b101_010"###);
    assert!(
        parse(&u128::MAX.to_string()).is_err(),
        "Parser should error on overflow"
    )
}

#[test]
fn string() {
    assert_stmt(
        Node::string((Span(0..7), String::from("ababa"))),
        r###""ababa""###,
    );
    assert_stmt(
        Node::string((Span(0..9), String::from("aba\"ba"))),
        r###""aba\"ba""###,
    );
    assert_stmt(
        Node::string((Span(0..11), String::from("aba\\\"ba"))),
        r###""aba\\\"ba""###,
    );
    assert_stmt(
        Node::string((Span(0..16), String::from(r#"aba#aba"aba"#))),
        r###"r#"aba#aba"aba"#"###,
    );
    assert_stmt(
        Node::string((Span(0..20), String::from(r##"aba"#aba#"aba"##))),
        r###"r##"aba"#aba#"aba"##"###,
    );
}

#[test]
fn identifier() {
    assert_stmt(Node::id((Span(0..5), "ababa")), r###"ababa"###);
    assert_stmt(Node::id((Span(0..2), "da")), r###"da"###);
    assert!(
        parse(r###"d"###).is_err(),
        "Parser should not accept `d` as id"
    )
}
