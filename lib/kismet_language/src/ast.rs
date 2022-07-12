use std::{error::Error, fmt};

use lalrpop_util::ParseError as LalrpopError;

use super::lexer::LexerError;
use super::token::Token;
use super::types::Integer;

pub type ParseResult<'input> = Result<Node<'input>, ParseError<'input>>;
pub type ParseError<'input> = LalrpopError<usize, Token<'input>, LexerError>;

#[derive(Debug)]
pub enum Node<'input> {
    Stmts(Vec<Node<'input>>),
    Op(Box<Node<'input>>, Token<'input>, Box<Node<'input>>),
    Enclosure(Token<'input>, Box<Node<'input>>, Token<'input>),
    Unary(Token<'input>, Box<Node<'input>>),
    Tuple(Vec<Node<'input>>),
    Id(&'input str),
    String(String),
    Integer(Integer),
    Error(Box<dyn Error>),
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
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_seq(f: &mut fmt::Formatter, nodes: &Vec<Node>, delim: &'static str) -> fmt::Result {
            write!(
                f,
                "{}",
                nodes
                    .iter()
                    .map(|node| node.to_string())
                    .collect::<Vec<String>>()
                    .join(delim)
            )
        }

        match self {
            Node::Stmts(nodes) => fmt_seq(f, nodes, "\n"),
            Node::Tuple(nodes) => {
                fmt_seq(f, nodes, ", ")?;
                match nodes.len() {
                    0 => write!(f, "()"),
                    1 => write!(f, ","),
                    _ => Ok(()),
                }
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
            Node::Op(left, op, right) => match (left.is_int() || left.is_tuple(), op) {
                (false, Token::DIE) => {
                    write!(f, "({}){}{}{}{}", left, op.space(), op, op.space(), right)
                }
                _ => write!(f, "{}{}{}{}{}", left, op.space(), op, op.space(), right),
            },
            Node::Unary(op, right) => {
                write!(f, "{}{}{}", op, op.space(), right)
            }
            Node::String(s) => write!(f, "\"{}\"", s),
            Node::Integer(n) => write!(f, "{}", n),
            Node::Id(s) => write!(f, "{}", s),
            Node::Error(e) => write!(f, "{}", e),
        }
    }
}
