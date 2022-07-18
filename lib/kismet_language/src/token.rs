use std::fmt;
use std::ops::Deref;

use logos::{Lexer, Logos};
use syn::{parse_str, LitFloat, LitInt, LitStr};

use crate::ast::{Atom, Expr};
use crate::types::{Float, Integer, Span};

pub type Token<'input> = BaseToken<TokenKind<'input>>;

#[derive(Clone, Debug, PartialEq)]
pub struct BaseToken<Kind> {
    pub span: Span,
    pub kind: Kind,
}

impl<'input, Kind> BaseToken<Kind> {
    pub fn vec_to_span(v: &'input Vec<BaseToken<Kind>>) -> Option<Span> {
        Span::reduce(&mut v.iter().map(|x| x.span.clone()))
    }
}

impl<Kind> Deref for BaseToken<Kind> {
    type Target = Kind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl<Kind: std::fmt::Display> fmt::Display for BaseToken<Kind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum TokenKind<'input> {
    #[regex(r"[;\n]")]
    DELIM,

    #[token(",")]
    COMMA,

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

    #[regex("\"", TokenKind::parse_string)]
    #[regex("r#*\"", TokenKind::parse_rawstring)]
    String(String),

    #[regex(r"[[:digit:]][[:digit:]_]*", TokenKind::parse_number)]
    #[regex(r"0b[0-1_]*", TokenKind::parse_int)]
    #[regex(r"0o[0-7_]*", TokenKind::parse_int)]
    #[regex(r"0x[[:xdigit:]_]*", TokenKind::parse_int)]
    Number(NumberKind),

    #[regex(
        r"([[:alpha:]--[dD]_]|[dD][[:alpha:]_])[[:word:]]*",
        TokenKind::parse_id
    )]
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

impl<'input> TokenKind<'input> {
    fn parse_id(t: &mut Lexer<'input, TokenKind<'input>>) -> &'input str {
        t.slice()
    }

    fn parse_number(t: &mut Lexer<'input, TokenKind<'input>>) -> Result<NumberKind, ()> {
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
            true => TokenKind::parse_float(t),
            false => TokenKind::parse_int(t),
        }
    }

    fn parse_int(t: &mut Lexer<'input, TokenKind<'input>>) -> Result<NumberKind, ()> {
        match parse_str::<LitInt>(t.slice()) {
            Ok(n) => match n.base10_parse::<Integer>() {
                Ok(i) => Ok(NumberKind::Integer(i)),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    fn parse_float(t: &mut Lexer<'input, TokenKind<'input>>) -> Result<NumberKind, ()> {
        match parse_str::<LitFloat>(t.slice()) {
            Ok(n) => match n.base10_parse::<Float>() {
                Ok(i) => Ok(NumberKind::Float(i)),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    fn parse_string(t: &mut Lexer<'input, TokenKind<'input>>) -> Result<String, ()> {
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

    fn parse_rawstring(t: &mut Lexer<'input, TokenKind<'input>>) -> Result<String, ()> {
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
            TokenKind::DIE
            | TokenKind::POW
            | TokenKind::MUL
            | TokenKind::LPAREN
            | TokenKind::RPAREN => "",
            _ => " ",
        }
    }
}

impl fmt::Display for TokenKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::DELIM => write!(f, "\n"),
            TokenKind::COMMA => write!(f, ","),
            TokenKind::SPREAD => write!(f, "..."),
            TokenKind::FOR => write!(f, "FOR"),
            TokenKind::IN => write!(f, "IN"),
            TokenKind::IF => write!(f, "IF"),
            TokenKind::AND => write!(f, "AND"),
            TokenKind::OR => write!(f, "OR"),
            TokenKind::NOT => write!(f, "NOT"),
            TokenKind::EQ => write!(f, "=="),
            TokenKind::NE => write!(f, "!="),
            TokenKind::LT => write!(f, "<"),
            TokenKind::LE => write!(f, "<="),
            TokenKind::GT => write!(f, ">"),
            TokenKind::GE => write!(f, ">="),
            TokenKind::ADD => write!(f, "+"),
            TokenKind::SUB => write!(f, "-"),
            TokenKind::MOD => write!(f, "%"),
            TokenKind::MUL => write!(f, "*"),
            TokenKind::DIV => write!(f, "/"),
            TokenKind::POW => write!(f, "^"),
            TokenKind::DIE => write!(f, "d"),
            TokenKind::LPAREN => write!(f, "("),
            TokenKind::RPAREN => write!(f, ")"),
            TokenKind::LBRACKET => write!(f, "["),
            TokenKind::RBRACKET => write!(f, "]"),
            TokenKind::LBRACE => write!(f, "{{"),
            TokenKind::RBRACE => write!(f, "}}"),
            TokenKind::String(value) => write!(f, r#""{}""#, value),
            TokenKind::Number(value) => write!(f, "{}", value),
            TokenKind::Id(value) => write!(f, "{}", value),
            TokenKind::SKIP | TokenKind::ERROR => write!(f, "{:?}", self),
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
