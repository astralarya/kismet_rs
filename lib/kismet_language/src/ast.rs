use std::{error::Error, fmt};

use lalrpop_util::ParseError as LalrpopError;

use super::lexer::{LexerError, Token};

pub type ParseResult<'input> = Result<Node<'input>, ParseError<'input>>;
pub type ParseError<'input> = LalrpopError<usize, Token<'input>, LexerError>;

#[derive(Debug)]
pub enum Node<'input> {
    Stmts(Vec<Node<'input>>),
    Op(Box<Node<'input>>, Token<'input>, Box<Node<'input>>),
    Unary(Token<'input>, Box<Node<'input>>),
    Group(Token<'input>, Box<Node<'input>>, Token<'input>),
    Id(&'input str),
    Int(i32),
    Error(Box<dyn Error>),
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Stmts(v) => {
                for (idx, n) in v.iter().enumerate() {
                    match idx {
                        0 => write!(f, "{}", n)?,
                        _ => write!(f, "\n{}", n)?,
                    }
                }
                Ok(())
            }
            Node::Op(left, op, right) => {
                write!(f, "{}{}{}{}{}", left, op.space(), op, op.space(), right)
            }
            Node::Unary(op, right) => {
                write!(f, "{}{}{}", op, op.space(), right)
            }
            Node::Group(left, node, right) => {
                write!(f, "{}{}{}", left, node, right)
            }
            Node::Int(n) => write!(f, "{}", n),
            Node::Id(s) => write!(f, "{}", s),
            Node::Error(e) => write!(f, "{}", e),
        }
    }
}
