use kismet_language::{ast::Node, parse};

fn assert_node_eq(node: Node, str: &str) {
    assert_eq!(Ok(Node::Stmts(vec![node])), parse(str))
}

#[test]
fn integer() {
    assert_node_eq(Node::Integer(42), r###"42"###);
    assert_node_eq(Node::Integer(42), r###"4_2"###);
    assert_node_eq(Node::Integer(42), r###"0x2A"###);
    assert_node_eq(Node::Integer(42), r###"0x2_A"###);
    assert_node_eq(Node::Integer(42), r###"0o52"###);
    assert_node_eq(Node::Integer(42), r###"0o5_2"###);
    assert_node_eq(Node::Integer(42), r###"0b101010"###);
    assert_node_eq(Node::Integer(42), r###"0b101_010"###);
    assert!(
        parse(&i128::MAX.to_string()).is_err(),
        "Parser should error on overflow"
    )
}

#[test]
fn string() {
    assert_node_eq(Node::String(String::from("ababa")), r###""ababa""###);
    assert_node_eq(Node::String(String::from("aba\"ba")), r###""aba\"ba""###);
    assert_node_eq(
        Node::String(String::from("aba\\\"ba")),
        r###""aba\\\"ba""###,
    );
    assert_node_eq(
        Node::String(String::from(r#"aba#aba"aba"#)),
        r###"r#"aba#aba"aba"#"###,
    );
    assert_node_eq(
        Node::String(String::from(r##"aba"#aba#"aba"##)),
        r###"r##"aba"#aba#"aba"##"###,
    );
}

#[test]
fn identifier() {
    assert_node_eq(Node::Id("ababa"), r###"ababa"###);
    assert_node_eq(Node::Id("da"), r###"da"###);
    assert!(
        parse(r###"d"###).is_err(),
        "Parser should not accept `d` as id"
    )
}
