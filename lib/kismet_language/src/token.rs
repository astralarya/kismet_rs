use std::fmt;

use logos::{Lexer, Logos};
use syn::{parse_str, LitInt, LitStr};

use super::ast::NodeKind;
use super::types::Integer;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    #[regex(r"[;\n]")]
    DELIM,

    #[regex(",")]
    COMMA,

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

    #[regex("\"", Token::parse_string)]
    #[regex("r#*\"", Token::parse_rawstring)]
    String(String),

    #[regex(r"[[:digit:]][[:digit:]_]*", Token::parse_int)]
    #[regex(r"0b[0-1_]*", Token::parse_int)]
    #[regex(r"0o[0-7_]*", Token::parse_int)]
    #[regex(r"0x[[:xdigit:]_]*", Token::parse_int)]
    Integer(Integer),

    #[regex(r"([[:alpha:]--[dD]_]|[dD][[:alpha:]_])[[:word:]]*")]
    Id(&'input str),

    #[regex(r"[ \t\f]+", logos::skip)]
    SKIP,

    #[error]
    ERROR,
}

impl<'input> Token<'input> {
    fn parse_int(t: &mut Lexer<'input, Token<'input>>) -> Result<Integer, ()> {
        match parse_str::<LitInt>(t.slice()) {
            Ok(n) => match n.base10_parse::<Integer>() {
                Ok(i) => Ok(i),
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

        let mut string = String::from(t.slice());
        let remainder = t.remainder();
        for token in Part::lexer(&t.remainder()) {
            match token {
                Part::Quote => {
                    t.bump(1);
                    string.push_str(&remainder[0..remainder.len() - &t.remainder().len()]);
                    return match parse_str::<LitStr>(string.as_str()) {
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

        let mut string = String::from(t.slice());
        let remainder = t.remainder();
        let guard = string.len() - 2;
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
                        string.push_str(&remainder[0..remainder.len() - &t.remainder().len()]);
                        return match parse_str::<LitStr>(string.as_str()) {
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

    pub fn enclose(&self, node: &Box<NodeKind<'input>>) -> bool {
        match (
            self,
            !node.is_int() && !node.is_tuple() && !node.is_vector(),
        ) {
            (Token::DIE, true) => true,
            _ => false,
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
            Token::LBRACKET => write!(f, "["),
            Token::RBRACKET => write!(f, "]"),
            Token::LBRACE => write!(f, "{{"),
            Token::RBRACE => write!(f, "}}"),
            _ => write!(f, "{:?}", self),
        }
    }
}
