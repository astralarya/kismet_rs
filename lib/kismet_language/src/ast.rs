use std::fmt;

use lalrpop_util::ParseError as LalrpopError;

use super::lexer::LexerError;
use super::token::Token;
use super::types::Integer;

pub type ParseResult<'input> = Result<Node<'input>, ParseError<'input>>;
pub type ParseError<'input> = LalrpopError<usize, Token<'input>, LexerError>;

#[derive(Debug, PartialEq)]
pub struct Node<'input> {
    pub kind: Box<NodeKind<'input>>,
}

#[derive(Debug, PartialEq)]
pub enum NodeKind<'input> {
    Stmts(Vec<Node<'input>>),
    Comprehension(Node<'input>, Vec<Node<'input>>),
    CompFor(Node<'input>, Node<'input>, Option<Node<'input>>),
    TargetList(Vec<Node<'input>>),
    Op(Node<'input>, Token<'input>, Node<'input>),
    Unary(Token<'input>, Node<'input>),
    Enclosure(Token<'input>, Node<'input>, Token<'input>),
    Vector(Vec<Node<'input>>),
    Tuple(Vec<Node<'input>>),
    Id(&'input str),
    String(String),
    Integer(Integer),
}

impl<'input> Node<'input> {
    pub fn stmts(v: Vec<Node<'input>>) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::Stmts(v)),
        };
    }

    pub fn comprehension(n: Node<'input>, v: Vec<Node<'input>>) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::Comprehension(n, v)),
        };
    }

    pub fn comp_for(
        item: Node<'input>,
        iter: Node<'input>,
        ifnode: Option<Node<'input>>,
    ) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::CompFor(item, iter, ifnode)),
        };
    }

    pub fn target_list(v: Vec<Node<'input>>) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::TargetList(v)),
        };
    }

    pub fn op(l: Node<'input>, o: Token<'input>, r: Node<'input>) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::Op(l, o, r)),
        };
    }

    pub fn unary(o: Token<'input>, r: Node<'input>) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::Unary(o, r)),
        };
    }

    pub fn enclosure(l: Token<'input>, n: Node<'input>, r: Token<'input>) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::Enclosure(l, n, r)),
        };
    }

    pub fn vector(v: Vec<Node<'input>>) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::Vector(v)),
        };
    }

    pub fn tuple(v: Vec<Node<'input>>) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::Tuple(v)),
        };
    }

    pub fn id(i: &'input str) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::Id(i)),
        };
    }

    pub fn string(i: String) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::String(i)),
        };
    }

    pub fn integer(i: Integer) -> Node<'input> {
        return Node {
            kind: Box::new(NodeKind::Integer(i)),
        };
    }
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn join(nodes: &Vec<Node>, delim: &'static str) -> String {
            nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join(delim)
        }

        match &*self.kind {
            NodeKind::Stmts(nodes) => write!(f, "{}", join(&nodes, "\n")),
            NodeKind::Comprehension(l, v) => write!(f, "{} {}", l, join(&v, " ")),
            NodeKind::CompFor(item, iter, expr) => match expr {
                Some(node) => write!(f, "FOR {} IN {} IF {}", item, iter, node),
                None => write!(f, "FOR {} IN {}", item, iter),
            },
            NodeKind::TargetList(v) => write!(f, "{}", join(&v, ", ")),
            NodeKind::Op(left, op, right) => {
                match (op.enclose(&*left.kind), op.enclose(&*right.kind)) {
                    (true, true) => {
                        write!(f, "({}){}{}{}({})", left, op.space(), op, op.space(), right)
                    }
                    (true, false) => {
                        write!(f, "({}){}{}{}{}", left, op.space(), op, op.space(), right)
                    }
                    (false, true) => {
                        write!(f, "{}{}{}{}({})", left, op.space(), op, op.space(), right)
                    }
                    (false, false) => {
                        write!(f, "{}{}{}{}{}", left, op.space(), op, op.space(), right)
                    }
                }
            }
            NodeKind::Unary(op, right) => match op.enclose(&*right.kind) {
                true => write!(f, "{}{}({})", op, op.space(), right),
                false => write!(f, "{}{}{}", op, op.space(), right),
            },
            NodeKind::Enclosure(left, op, right) => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    left,
                    left.space(),
                    op,
                    right.space(),
                    right
                )
            }
            NodeKind::Vector(nodes) => write!(f, "[{}]", join(&nodes, ", ")),
            NodeKind::Tuple(nodes) => match nodes.len() {
                1 => write!(f, "({},)", nodes[0]),
                _ => write!(f, "({})", join(&nodes, ", ")),
            },
            NodeKind::String(s) => write!(f, "\"{}\"", s),
            NodeKind::Integer(n) => write!(f, "{}", n),
            NodeKind::Id(s) => write!(f, "{}", s),
        }
    }
}
