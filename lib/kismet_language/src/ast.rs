use std::{error::Error, fmt};

use lalrpop_util::{lexer::Token, ParseError as LalrpopError};

pub type ParseResult<'life> = Result<Node, ParseError<'life>>;
pub type ParseError<'life> = LalrpopError<usize, Token<'life>, &'life str>;

pub enum Node {
    Op(Box<Node>, Sym, Box<Node>),
    Unary(Sym, Box<Node>),
    Paren(Box<Node>),
    Int(i32),
    Error(Box<dyn Error>),
}

pub enum Sym {
    Die,
    Mod,
    Mul,
    Div,
    Add,
    Sub,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(n) => write!(f, "{}", n),
            Node::Paren(e) => write!(f, "({})", e),
            Node::Op(l, o, r) => match o {
                Sym::Die | Sym::Mul => write!(f, "{}{}{}", l, o, r),
                _ => write!(f, "{} {} {}", l, o, r),
            },
            Node::Unary(o, r) => match o {
                _ => write!(f, "{}{}", o, r),
            },
            Node::Error(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sym::Die => write!(f, "d"),
            Sym::Mod => write!(f, "%"),
            Sym::Mul => write!(f, "*"),
            Sym::Div => write!(f, "/"),
            Sym::Add => write!(f, "+"),
            Sym::Sub => write!(f, "-"),
        }
    }
}
