use std::fmt;

use logos::{Lexer, Logos};
use nom::Err;
use syn::{parse_str, LitFloat, LitInt, LitStr};

use crate::{
    ast::Node,
    types::{Float, Integer},
};

use super::{Error, ErrorKind, KResult};

pub fn token<'input>(input: Node<&'input str>) -> KResult<Node<&'input str>, Node<Token<'input>>> {
    let mut lexer = Token::lexer(&input.data);
    match lexer.next() {
        Some(Token::ERROR) => Err(Err::Error(Error {
            input,
            code: ErrorKind::Lex,
        })),
        Some(val) => {
            let start = input.span.len() - lexer.remainder().len();
            Ok((
                Node::new(input.span.slice(start..), lexer.remainder()),
                Node::new(input.span.slice(..start), val),
            ))
        }
        None => Err(Err::Error(Error {
            input: input,
            code: ErrorKind::Eof,
        })),
    }
}

pub fn token_if<'input, P>(
    input: Node<&'input str>,
    predicate: P,
) -> KResult<Node<&'input str>, Node<Token<'input>>>
where
    P: Fn(Node<Token>) -> bool,
{
    let (tail, head) = token(input.clone())?;
    match predicate(head.clone()) {
        true => Ok((tail, head)),
        false => Err(Err::Error(Error {
            input,
            code: ErrorKind::Predicate,
        })),
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    #[regex(r"[;\n]")]
    DELIM,

    #[token(",")]
    COMMA,

    #[token(":")]
    COLON,

    #[token("...")]
    SPREAD,

    #[regex(r"(?i)for")]
    FOR,

    #[regex(r"(?i)in")]
    IN,

    #[regex(r"(?i)if")]
    IF,

    #[regex(r"(?i)or")]
    OR,

    #[regex(r"(?i)and")]
    AND,

    #[regex(r"(?i)not")]
    NOT,

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

    #[token(".")]
    DOT,

    #[token("(")]
    LPAREN,

    #[token(")")]
    RPAREN,

    #[token("[")]
    LBRACKET,

    #[token("]")]
    RBRACKET,

    #[token("{")]
    LBRACE,

    #[token("}")]
    RBRACE,

    #[regex("\"", Token::parse_string)]
    #[regex("r#*\"", Token::parse_rawstring)]
    String(String),

    #[regex(r"[[:digit:]][[:digit:]_]*", Token::parse_number)]
    #[regex(r"0b[0-1_]*", Token::parse_int)]
    #[regex(r"0o[0-7_]*", Token::parse_int)]
    #[regex(r"0x[[:xdigit:]_]*", Token::parse_int)]
    Number(NumberKind),

    #[regex(r"([[:alpha:]--[dD]_]|[dD][[:alpha:]_])[[:word:]]*", Token::parse_id)]
    Id(&'input str),

    #[regex(r"[ \t\f]+", logos::skip)]
    SKIP,

    #[error]
    ERROR,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberKind {
    Integer(Integer),
    Float(Float),
}

impl<'input> Token<'input> {
    fn parse_id(t: &mut Lexer<'input, Token<'input>>) -> &'input str {
        t.slice()
    }

    fn parse_number(t: &mut Lexer<'input, Token<'input>>) -> Result<NumberKind, ()> {
        #[derive(Logos, Debug, PartialEq)]
        enum Part<'input> {
            #[regex(r"\.([[:digit:]][[:digit:]_]*)?")]
            Dot(&'input str),

            #[regex(r"[eE][+-]?_*[[:digit:]][[:digit:]_]*")]
            Exponent(&'input str),

            #[regex(r"\.[[:alpha:]_.]")]
            Break,

            #[error]
            #[regex(r"", logos::skip)]
            Error,
        }

        let mut dot = false;
        let mut exp = false;
        for token in Part::lexer(&t.remainder()) {
            match (token, dot, exp) {
                (Part::Dot(s), false, _) => {
                    t.bump(s.len());
                    dot = true;
                }
                (Part::Exponent(s), _, false) => {
                    t.bump(s.len());
                    exp = true;
                    dot = false;
                }
                _ => {
                    break;
                }
            }
        }
        match dot || exp {
            true => Token::parse_float(t),
            false => Token::parse_int(t),
        }
    }

    fn parse_int(t: &mut Lexer<'input, Token<'input>>) -> Result<NumberKind, ()> {
        match parse_str::<LitInt>(t.slice()) {
            Ok(n) => match n.base10_parse::<Integer>() {
                Ok(i) => Ok(NumberKind::Integer(i)),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    fn parse_float(t: &mut Lexer<'input, Token<'input>>) -> Result<NumberKind, ()> {
        match parse_str::<LitFloat>(t.slice()) {
            Ok(n) => match n.base10_parse::<Float>() {
                Ok(i) => Ok(NumberKind::Float(i)),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    fn parse_string(t: &mut Lexer<'input, Token<'input>>) -> Result<String, ()> {
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

        for token in Part::lexer(&t.remainder()) {
            match token {
                Part::Quote => {
                    t.bump(1);
                    return match parse_str::<LitStr>(t.slice()) {
                        Ok(n) => Ok(n.value()),
                        Err(_) => Err(()),
                    };
                }
                Part::Chars(s) => t.bump(s.len()),
                Part::SlashEscape => t.bump(2),
                Part::Error => return Err(()),
            }
        }
        Err(())
    }

    fn parse_rawstring(t: &mut Lexer<'input, Token<'input>>) -> Result<String, ()> {
        #[derive(Logos, Debug, PartialEq)]
        enum Part<'input> {
            #[token("\"")]
            Quote,

            #[token("#")]
            Hash,

            #[regex(r##"[^"#]+"##)]
            Chars(&'input str),

            #[error]
            #[regex(r"", logos::skip)]
            Error,
        }

        let guard = t.span().end - t.span().start - 2;
        let mut signal: Option<usize> = None;
        for token in Part::lexer(&t.remainder()) {
            match token {
                Part::Quote => {
                    t.bump(1);
                    signal = Some(0);
                }
                Part::Hash => {
                    t.bump(1);
                    match signal {
                        Some(s) => signal = Some(s + 1),
                        None => (),
                    }
                }
                Part::Chars(s) => {
                    t.bump(s.len());
                    signal = None;
                }
                Part::Error => return Err(()),
            }
            match signal {
                Some(signal_val) => match (signal_val == guard, token) {
                    (true, Part::Quote) | (true, Part::Hash) => {
                        return match parse_str::<LitStr>(t.slice()) {
                            Ok(n) => Ok(n.value()),
                            Err(_) => Err(()),
                        };
                    }
                    _ => (),
                },
                None => (),
            }
        }
        Err(())
    }

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
            Token::COLON => write!(f, ":"),
            Token::SPREAD => write!(f, "..."),
            Token::FOR => write!(f, "FOR"),
            Token::IN => write!(f, "IN"),
            Token::IF => write!(f, "IF"),
            Token::AND => write!(f, "AND"),
            Token::OR => write!(f, "OR"),
            Token::NOT => write!(f, "NOT"),
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
            Token::DOT => write!(f, "."),
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            Token::LBRACKET => write!(f, "["),
            Token::RBRACKET => write!(f, "]"),
            Token::LBRACE => write!(f, "{{"),
            Token::RBRACE => write!(f, "}}"),
            Token::String(value) => write!(f, r#""{}""#, value),
            Token::Number(value) => write!(f, "{}", value),
            Token::Id(value) => write!(f, "{}", value),
            Token::SKIP | Token::ERROR => write!(f, "{:?}", self),
        }
    }
}

impl fmt::Display for NumberKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumberKind::Float(value) => write!(f, "{}", value),
            NumberKind::Integer(value) => write!(f, "{}", value),
        }
    }
}
