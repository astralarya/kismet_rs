use std::{fmt, str::FromStr};

use logos::{Lexer, Logos};

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    #[regex(r"[;\n]")]
    DELIM,

    #[regex(r"(?i)or")]
    OR,

    #[regex(r"(?i)and")]
    AND,

    #[token("==")]
    EQ,

    #[token("!=")]
    NE,

    #[token("<")]
    LT,

    #[token("<=")]
    LE,

    #[token(">")]
    GT,

    #[token(">=")]
    GE,

    #[token("+")]
    ADD,

    #[token("-")]
    SUB,

    #[token("%")]
    MOD,

    #[token("*")]
    MUL,

    #[token("/")]
    DIV,

    #[token("^")]
    POW,

    #[regex(r"(?i)d")]
    DIE,

    #[token("(")]
    LPAREN,

    #[token(")")]
    RPAREN,

    #[regex(r"[0-9]+", parse_int)]
    Int(i32),

    #[regex(r"([_a-ce-zA-CE-Z]|d[_a-zA-Z])[_a-zA-Z0-9]*")]
    Id(&'input str),

    #[regex(r"[ \t\f]+", logos::skip)]
    SKIP,

    #[error]
    ERROR,
}

impl<'input> Token<'input> {
    pub fn space(&self) -> &'static str {
        match self {
            Token::DIE | Token::POW | Token::MUL => "",
            _ => " ",
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::DELIM => write!(f, "\n"),
            Token::OR => write!(f, "OR"),
            Token::AND => write!(f, "AND"),
            Token::EQ => write!(f, "=="),
            Token::NE => write!(f, "!="),
            Token::LT => write!(f, "<"),
            Token::LE => write!(f, "<="),
            Token::GT => write!(f, ">"),
            Token::GE => write!(f, ">="),
            Token::ADD => write!(f, "+"),
            Token::SUB => write!(f, "-"),
            Token::MOD => write!(f, "%"),
            Token::MUL => write!(f, "*"),
            Token::DIV => write!(f, "/"),
            Token::POW => write!(f, "^"),
            Token::DIE => write!(f, "d"),
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            _ => write!(f, "{:?}", self),
        }
    }
}

fn parse_int<'input>(t: &mut Lexer<'input, Token<'input>>) -> Result<i32, ()> {
    match i32::from_str(t.slice()) {
        Ok(i) => Ok(i),
        Err(_) => Err(()),
    }
}
