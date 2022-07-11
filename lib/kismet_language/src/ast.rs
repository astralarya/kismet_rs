use std::{error::Error, fmt};

use lalrpop_util::ParseError as LalrpopError;

use super::lexer::Token;

pub type ParseResult<'input> = Result<Node<'input>, ParseError<'input>>;
pub type ParseError<'input> = LalrpopError<usize, Token<'input>, &'input str>;

#[derive(Debug)]
pub enum Node<'input> {
    Op(Box<Node<'input>>, Token<'input>, Box<Node<'input>>),
    Unary(Token<'input>, Box<Node<'input>>),
    Paren(Box<Node<'input>>),
    Id(&'input str),
    Int(i32),
    Error(Box<dyn Error>),
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(n) => write!(f, "{}", n),
            Node::Id(s) => write!(f, "{}", s),
            Node::Paren(e) => write!(f, "({})", e),
            Node::Op(l, o, r) => match o {
                Token::DIE | Token::POW | Token::MUL => write!(f, "{}{}{}", l, o, r),
                _ => write!(f, "{} {} {}", l, o, r),
            },
            Node::Unary(o, r) => match o {
                _ => write!(f, "{}{}", o, r),
            },
            Node::Error(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            Token::DIE => write!(f, "d"),
            Token::POW => write!(f, "^"),
            Token::MOD => write!(f, "%"),
            Token::MUL => write!(f, "*"),
            Token::DIV => write!(f, "/"),
            Token::ADD => write!(f, "+"),
            Token::SUB => write!(f, "-"),
            Token::EQ => write!(f, "=="),
            Token::NE => write!(f, "!="),
            Token::LT => write!(f, "<"),
            Token::LE => write!(f, "<="),
            Token::GT => write!(f, ">"),
            Token::GE => write!(f, ">="),
            Token::AND => write!(f, "AND"),
            Token::OR => write!(f, "OR"),
            t => write!(f, "{}", t),
        }
    }
}
