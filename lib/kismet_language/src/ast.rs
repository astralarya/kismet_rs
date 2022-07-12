use std::{error::Error, fmt};

use lalrpop_util::ParseError as LalrpopError;

use super::lexer::LexerError;
use super::token::Token;

pub type ParseResult<'input> = Result<Node<'input>, ParseError<'input>>;
pub type ParseError<'input> = LalrpopError<usize, Token<'input>, LexerError>;

#[derive(Debug)]
pub enum Node<'input> {
    Stmts(Vec<Node<'input>>),
    Exprs(Vec<Node<'input>>),
    Op(Box<Node<'input>>, Token<'input>, Box<Node<'input>>),
    Unary(Token<'input>, Box<Node<'input>>),
    Tuple(Box<Node<'input>>),
    Id(&'input str),
    Int(i32),
    Error(Box<dyn Error>),
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
            Node::Exprs(nodes) => fmt_seq(f, nodes, ", "),
            Node::Op(left, op, right) => {
                write!(f, "{}{}{}{}{}", left, op.space(), op, op.space(), right)
            }
            Node::Unary(op, right) => {
                write!(f, "{}{}{}", op, op.space(), right)
            }
            Node::Tuple(node) => write!(f, "({})", node),
            Node::Int(n) => write!(f, "{}", n),
            Node::Id(s) => write!(f, "{}", s),
            Node::Error(e) => write!(f, "{}", e),
        }
    }
}
