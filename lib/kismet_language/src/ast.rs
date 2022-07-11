use std::{error::Error, fmt};

use lalrpop_util::ParseError as LalrpopError;

use super::lexer::{LexerError, Token};

pub type ParseResult<'input> = Result<Node<'input>, ParseError<'input>>;
pub type ParseError<'input> = LalrpopError<usize, Token<'input>, LexerError>;

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
        fn op_str(op: &Token) -> Option<&'static str> {
            match op {
                Token::OR => Some("OR"),
                Token::AND => Some("AND"),
                Token::EQ => Some("=="),
                Token::NE => Some("!="),
                Token::LT => Some("<"),
                Token::LE => Some("<="),
                Token::GT => Some(">"),
                Token::GE => Some(">="),
                Token::ADD => Some("+"),
                Token::SUB => Some("-"),
                Token::MOD => Some("%"),
                Token::MUL => Some("*"),
                Token::DIV => Some("/"),
                Token::POW => Some("^"),
                Token::DIE => Some("d"),
                Token::LPAREN => Some("("),
                Token::RPAREN => Some(")"),
                _ => None,
            }
        }
        match self {
            Node::Int(n) => write!(f, "{}", n),
            Node::Id(s) => write!(f, "{}", s),
            Node::Paren(e) => write!(f, "({})", e),
            Node::Op(l, o, r) => match op_str(o) {
                Some(s) => match o {
                    Token::DIE | Token::POW | Token::MUL => write!(f, "{}{}{}", l, s, r),
                    _ => write!(f, "{} {} {}", l, s, r),
                },
                None => write!(f, "{} {} {}", l, o, r),
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
        write!(f, "{:?}", self)
    }
}
