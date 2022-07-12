use std::{fmt, str::FromStr};

use logos::{Lexer, Logos};

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    #[regex(r"[;\n]")]
    DELIM,

    #[regex(",")]
    COMMA,

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

    #[regex("\"", parse_string)]
    String(&'input str),

    #[regex(r"[0-9]+", parse_int)]
    Int(i32),

    #[regex(r"([[:alpha:]--[dD]_]|[dD][[:alpha:]_])[[:word:]]*")]
    Id(&'input str),

    #[regex(r"[ \t\f]+", logos::skip)]
    SKIP,

    #[error]
    ERROR,
}

impl<'input> Token<'input> {
    pub fn space(&self) -> &'static str {
        match self {
            Token::DIE | Token::POW | Token::MUL | Token::LPAREN | Token::RPAREN => "",
            _ => " ",
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::DELIM => write!(f, "\n"),
            Token::COMMA => write!(f, ","),
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

fn parse_string<'input>(t: &mut Lexer<'input, Token<'input>>) -> Result<&'input str, ()> {
    #[derive(Logos, Debug, PartialEq)]
    enum Part<'input> {
        #[token("\"")]
        Quote,

        #[regex(r#"[^"\\]+"#)]
        Chars(&'input str),

        #[regex(r#"\\."#)]
        SlashEscape,

        #[error]
        #[regex(r"", logos::skip)]
        Error,
    }

    let remainder = t.remainder();
    for token in Part::lexer(&t.remainder()) {
        match token {
            Part::Quote => {
                let string = &remainder[0..remainder.len() - &t.remainder().len()];
                t.bump(1);
                return Ok(string);
            }
            Part::Chars(s) => t.bump(s.len()),
            Part::SlashEscape => t.bump(2),
            Part::Error => return Err(()),
        }
    }
    Err(())
}
