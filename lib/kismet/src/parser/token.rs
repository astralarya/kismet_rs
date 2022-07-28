use std::fmt;

use logos::{Lexer, Logos, SpannedIter};
use nom::Err;
use syn::{parse_str, LitFloat, LitInt, LitStr};

use crate::types::{Float, Integer, Node, ONode};

use super::{ErrorKind, Input, KResult};

pub struct TokenIterator<'a> {
    iter: SpannedIter<'a, Token>,
}

impl<'a> TokenIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        TokenIterator {
            iter: Token::lexer(input).spanned(),
        }
    }
}

impl Iterator for TokenIterator<'_> {
    type Item = Node<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((token, range)) => Some(Node::new(range, token)),
            None => None,
        }
    }
}

pub fn token<'input>(i: Input<'input>) -> KResult<'input, &Node<Token>> {
    match i.get(0) {
        Some(x) => match *x.data {
            Token::ERROR => Err(Err::Error(ONode::new(x.span, ErrorKind::Lex))),
            _ => Ok((&i[1..], x)),
        },
        None => Err(Err::Error(ONode::new(None, ErrorKind::Eof))),
    }
}

pub fn token_if<'input, P>(predicate: P) -> impl Fn(Input<'input>) -> KResult<'input, &Node<Token>>
where
    P: Fn(&Node<Token>) -> bool,
{
    move |input| {
        let (tail, head) = token(input)?;
        match predicate(head) {
            true => Ok((tail, head)),
            false => Err(Err::Error(ONode::new(head.span, ErrorKind::Predicate))),
        }
    }
}

pub fn token_tag_id<'input>(input: Input<'input>) -> KResult<'input, Node<String>> {
    let (tail, head) = token(input)?;
    match &*head.data {
        Token::Id(val) => Ok((tail, Node::new(head.span, val.clone()))),
        _ => Err(Err::Error(ONode::new(head.span, ErrorKind::Predicate))),
    }
}

pub fn token_tag<'input>(tag: Token) -> impl Fn(Input<'input>) -> KResult<'input, &Node<Token>> {
    move |input| {
        let (tail, head) = token(input)?;
        match *head.data == tag {
            true => Ok((tail, head)),
            false => Err(Err::Error(ONode::new(head.span, ErrorKind::Predicate))),
        }
    }
}

pub fn token_action<'input, T, Q>(action: Q) -> impl Fn(Input<'input>) -> KResult<'input, T>
where
    Q: Fn(&Node<Token>) -> Option<T>,
{
    move |input| {
        let (tail, head) = token(input)?;
        match action(head) {
            Some(t) => Ok((tail, t)),
            None => Err(Err::Error(ONode::new(head.span, ErrorKind::Predicate))),
        }
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[;\n]")]
    DELIM,

    #[token(",")]
    COMMA,

    #[token(":")]
    COLON,

    #[token("=")]
    ASSIGN,

    #[token(":=")]
    ASSIGNE,

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

    #[token("..")]
    RANGE,

    #[token("..=")]
    RANGEI,

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
    Id(String),

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

impl Token {
    fn parse_id(t: &mut Lexer<Token>) -> String {
        t.slice().to_string()
    }

    fn parse_number(t: &mut Lexer<Token>) -> Result<NumberKind, ()> {
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
            true => Self::parse_float(t),
            false => Self::parse_int(t),
        }
    }

    fn parse_int(t: &mut Lexer<Token>) -> Result<NumberKind, ()> {
        match parse_str::<LitInt>(t.slice()) {
            Ok(n) => match n.base10_parse::<Integer>() {
                Ok(i) => Ok(NumberKind::Integer(i)),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    fn parse_float(t: &mut Lexer<Token>) -> Result<NumberKind, ()> {
        match parse_str::<LitFloat>(t.slice()) {
            Ok(n) => match n.base10_parse::<Float>() {
                Ok(i) => Ok(NumberKind::Float(i)),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    fn parse_string(t: &mut Lexer<Token>) -> Result<String, ()> {
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

    fn parse_rawstring(t: &mut Lexer<Token>) -> Result<String, ()> {
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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DELIM => write!(f, "\n"),
            Self::COMMA => write!(f, ","),
            Self::COLON => write!(f, ":"),
            Self::ASSIGN => write!(f, "="),
            Self::ASSIGNE => write!(f, ":="),
            Self::SPREAD => write!(f, "..."),
            Self::FOR => write!(f, "for"),
            Self::IN => write!(f, "in"),
            Self::IF => write!(f, "if"),
            Self::AND => write!(f, "and"),
            Self::OR => write!(f, "or"),
            Self::NOT => write!(f, "not"),
            Self::EQ => write!(f, "=="),
            Self::NE => write!(f, "!="),
            Self::LT => write!(f, "<"),
            Self::LE => write!(f, "<="),
            Self::GT => write!(f, ">"),
            Self::GE => write!(f, ">="),
            Self::RANGE => write!(f, ".."),
            Self::RANGEI => write!(f, "..="),
            Self::ADD => write!(f, "+"),
            Self::SUB => write!(f, "-"),
            Self::MOD => write!(f, "%"),
            Self::MUL => write!(f, "*"),
            Self::DIV => write!(f, "/"),
            Self::POW => write!(f, "^"),
            Self::DIE => write!(f, "d"),
            Self::DOT => write!(f, "."),
            Self::LPAREN => write!(f, "("),
            Self::RPAREN => write!(f, ")"),
            Self::LBRACKET => write!(f, "["),
            Self::RBRACKET => write!(f, "]"),
            Self::LBRACE => write!(f, "{{"),
            Self::RBRACE => write!(f, "}}"),
            Self::String(value) => write!(f, r#""{}""#, value),
            Self::Number(value) => write!(f, "{}", value),
            Self::Id(value) => write!(f, "{}", value),
            Self::SKIP | Self::ERROR => write!(f, "{:?}", self),
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
