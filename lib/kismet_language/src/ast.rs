use std::{error::Error, fmt};

use lalrpop_util::{lexer::Token, ParseError as LalrpopError};

pub type ParseResult<'life> = Result<Node, ParseError<'life>>;
pub type ParseError<'life> = LalrpopError<usize, Token<'life>, &'life str>;

#[derive(Debug)]
pub enum Node {
    Op(Box<Node>, Sym, Box<Node>),
    Unary(Sym, Box<Node>),
    Paren(Box<Node>),
    Int(i32),
    Error(Box<dyn Error>),
}

#[derive(Debug)]
pub enum Sym {
    Die,
    Pow,
    Mod,
    Mul,
    Div,
    Add,
    Sub,
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    And,
    Or,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(n) => write!(f, "{}", n),
            Node::Paren(e) => write!(f, "({})", e),
            Node::Op(l, o, r) => match o {
                Sym::Die | Sym::Pow | Sym::Mul => write!(f, "{}{}{}", l, o, r),
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
            Sym::Pow => write!(f, "^"),
            Sym::Mod => write!(f, "%"),
            Sym::Mul => write!(f, "*"),
            Sym::Div => write!(f, "/"),
            Sym::Add => write!(f, "+"),
            Sym::Sub => write!(f, "-"),
            Sym::Eq => write!(f, "=="),
            Sym::NotEq => write!(f, "!="),
            Sym::Less => write!(f, "<"),
            Sym::LessEq => write!(f, "<="),
            Sym::Greater => write!(f, ">"),
            Sym::GreaterEq => write!(f, ">="),
            Sym::And => write!(f, "AND"),
            Sym::Or => write!(f, "OR"),
        }
    }
}
