use std::fmt;

use lalrpop_util::ParseError as LalrpopError;

use super::lexer::LexerError;
use super::token::Token;
use super::types::Integer;

pub type ParseResult<'input> = Result<Node<'input>, ParseError<'input>>;
pub type ParseError<'input> = LalrpopError<usize, Token<'input>, LexerError>;

#[derive(Debug, PartialEq)]
pub enum Node<'input> {
    Stmts(Vec<Node<'input>>),
    Comprehension(Box<Node<'input>>, Vec<Node<'input>>),
    CompFor(Box<Node<'input>>, Box<Node<'input>>),
    TargetList(Vec<Node<'input>>),
    Op(Box<Node<'input>>, Token<'input>, Box<Node<'input>>),
    Unary(Token<'input>, Box<Node<'input>>),
    Enclosure(Token<'input>, Box<Node<'input>>, Token<'input>),
    Vector(Vec<Node<'input>>),
    Tuple(Vec<Node<'input>>),
    Id(&'input str),
    String(String),
    Integer(Integer),
}

impl<'input> Node<'input> {
    pub fn is_vector(&self) -> bool {
        match self {
            Node::Vector(_) => true,
            _ => false,
        }
    }

    pub fn is_tuple(&self) -> bool {
        match self {
            Node::Tuple(_) => true,
            _ => false,
        }
    }

    pub fn is_id(&self) -> bool {
        match self {
            Node::Id(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Node::Integer(_) => true,
            _ => false,
        }
    }

    pub fn to_op(l: Node<'input>, o: Token<'input>, r: Node<'input>) -> Node<'input> {
        Node::Op(Box::new(l), o, Box::new(r))
    }

    pub fn to_unary(o: Token<'input>, r: Node<'input>) -> Node<'input> {
        Node::Unary(o, Box::new(r))
    }

    pub fn to_enclosure(l: Token<'input>, n: Node<'input>, r: Token<'input>) -> Node<'input> {
        Node::Enclosure(l, Box::new(n), r)
    }

    pub fn to_comprehension(n: Node<'input>, v: Vec<Node<'input>>) -> Node<'input> {
        Node::Comprehension(Box::new(n), v)
    }

    pub fn to_compfor(l: Node<'input>, r: Node<'input>) -> Node<'input> {
        Node::CompFor(Box::new(l), Box::new(r))
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

        match self {
            Node::Stmts(nodes) => write!(f, "{}", join(nodes, "\n")),
            Node::Comprehension(l, v) => write!(f, "{} {}", l, join(v, " ")),
            Node::CompFor(l, r) => write!(f, "FOR {} IN {}", l, r),
            Node::TargetList(v) => write!(f, "{}", join(v, ", ")),
            Node::Op(left, op, right) => match (op.enclose(left), op.enclose(right)) {
                (true, true) => {
                    write!(f, "({}){}{}{}({})", left, op.space(), op, op.space(), right)
                }
                (true, false) => write!(f, "({}){}{}{}{}", left, op.space(), op, op.space(), right),
                (false, true) => write!(f, "{}{}{}{}({})", left, op.space(), op, op.space(), right),
                (false, false) => write!(f, "{}{}{}{}{}", left, op.space(), op, op.space(), right),
            },
            Node::Unary(op, right) => match op.enclose(right) {
                true => write!(f, "{}{}({})", op, op.space(), right),
                false => write!(f, "{}{}{}", op, op.space(), right),
            },
            Node::Enclosure(left, op, right) => {
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
            Node::Vector(nodes) => write!(f, "[{}]", join(nodes, ", ")),
            Node::Tuple(nodes) => match nodes.len() {
                1 => write!(f, "({},)", nodes[0]),
                _ => write!(f, "({})", join(nodes, ", ")),
            },
            Node::String(s) => write!(f, "\"{}\"", s),
            Node::Integer(n) => write!(f, "{}", n),
            Node::Id(s) => write!(f, "{}", s),
        }
    }
}
