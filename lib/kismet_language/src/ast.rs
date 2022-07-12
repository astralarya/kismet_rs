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
            Node::Op(left, op, right) => match (
                left.is_int() || left.is_tuple(),
                op,
                right.is_int() || right.is_tuple(),
            ) {
                (true, Token::DIE, false) => {
                    write!(f, "{}{}{}{}({})", left, op.space(), op, op.space(), right)
                }
                (false, Token::DIE, true) => {
                    write!(f, "({}){}{}{}{}", left, op.space(), op, op.space(), right)
                }
                (false, Token::DIE, false) => {
                    write!(f, "({}){}{}{}({})", left, op.space(), op, op.space(), right)
                }
                _ => write!(f, "{}{}{}{}{}", left, op.space(), op, op.space(), right),
            },
            Node::Unary(op, right) => {
                write!(f, "{}{}{}", op, op.space(), right)
            }
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
