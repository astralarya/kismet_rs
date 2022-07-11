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

fn to_str(op: &Token) -> Option<&'static str> {
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

fn get_token_space(op: &Token) -> &'static str {
    match op {
        Token::DIE | Token::POW | Token::MUL => "",
        _ => " ",
    }
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
                let space = get_token_space(op);
                match to_str(op) {
                    Some(op_str) => write!(f, "{}{}{}{}{}", left, space, op_str, space, right),
                    None => write!(f, "{}{}{}{}{}", left, space, op, space, right),
                }
            }
            Node::Unary(op, right) => {
                let space = get_token_space(op);
                match to_str(op) {
                    Some(op_str) => write!(f, "{}{}{}", op_str, space, right),
                    None => write!(f, "{}{}{}", op, space, right),
                }
            }
            Node::Group(left, node, right) => match (to_str(left), to_str(right)) {
                (Some(lefts), Some(rights)) => write!(f, "{}{}{}", lefts, node, rights),
                _ => write!(f, "<{}>({})<{}>", left, node, right),
            },
            Node::Int(n) => write!(f, "{}", n),
            Node::Id(s) => write!(f, "{}", s),
            Node::Error(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
