use std::fmt;

use logos::{Lexer, Logos};
use syn::{parse_str, LitInt, LitStr};

use super::ast::NodeKind;
use super::types::{Integer, Span};

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    #[regex(r"[;\n]", Token::range)]
    DELIM(Span),

    #[regex(",", Token::range)]
    COMMA(Span),

    #[regex(r"(?i)for", Token::range)]
    FOR(Span),

    #[regex(r"(?i)in", Token::range)]
    IN(Span),

    #[regex(r"(?i)if", Token::range)]
    IF(Span),

    #[regex(r"(?i)or", Token::range)]
    OR(Span),

    #[regex(r"(?i)and", Token::range)]
    AND(Span),

    #[regex(r"(?i)not", Token::range)]
    NOT(Span),

    #[token("==", Token::range)]
    EQ(Span),

    #[token("!=", Token::range)]
    NE(Span),

    #[token("<", Token::range)]
    LT(Span),

    #[token("<=", Token::range)]
    LE(Span),

    #[token(">", Token::range)]
    GT(Span),

    #[token(">=", Token::range)]
    GE(Span),

    #[token("+", Token::range)]
    ADD(Span),

    #[token("-", Token::range)]
    SUB(Span),

    #[token("%", Token::range)]
    MOD(Span),

    #[token("*", Token::range)]
    MUL(Span),

    #[token("/", Token::range)]
    DIV(Span),

    #[token("^", Token::range)]
    POW(Span),

    #[regex(r"(?i)d", Token::range)]
    DIE(Span),

    #[token("(", Token::range)]
    LPAREN(Span),

    #[token(")", Token::range)]
    RPAREN(Span),

    #[token("[", Token::range)]
    LBRACKET(Span),

    #[token("]", Token::range)]
    RBRACKET(Span),

    #[token("{", Token::range)]
    LBRACE(Span),

    #[token("}", Token::range)]
    RBRACE(Span),

    #[regex("\"", Token::parse_string)]
    #[regex("r#*\"", Token::parse_rawstring)]
    String(String),

    #[regex(r"[[:digit:]][[:digit:]_]*", Token::parse_int)]
    #[regex(r"0b[0-1_]*", Token::parse_int)]
    #[regex(r"0o[0-7_]*", Token::parse_int)]
    #[regex(r"0x[[:xdigit:]_]*", Token::parse_int)]
    Integer((Span, Integer)),

    #[regex(r"([[:alpha:]--[dD]_]|[dD][[:alpha:]_])[[:word:]]*")]
    Id(&'input str),

    #[regex(r"[ \t\f]+", logos::skip)]
    SKIP,

    #[error]
    ERROR,
}

impl<'input> Token<'input> {
    fn range(t: &mut Lexer<'input, Token<'input>>) -> Span {
        t.span()
    }

    fn parse_int(t: &mut Lexer<'input, Token<'input>>) -> Result<(Span, Integer), ()> {
        match parse_str::<LitInt>(t.slice()) {
            Ok(n) => match n.base10_parse::<Integer>() {
                Ok(i) => Ok((t.span(), i)),
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
            Token::DIE(_) | Token::POW(_) | Token::MUL(_) | Token::LPAREN(_) | Token::RPAREN(_) => {
                ""
            }
            _ => " ",
        }
    }

    pub fn enclose(&self, kind: &NodeKind<'input>) -> bool {
        match (self, kind) {
            (Token::DIE(_), NodeKind::Integer(_))
            | (Token::DIE(_), NodeKind::Tuple(_))
            | (Token::DIE(_), NodeKind::Vector(_)) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::DELIM(_) => write!(f, "\n"),
            Token::COMMA(_) => write!(f, ","),
            Token::OR(_) => write!(f, "OR"),
            Token::AND(_) => write!(f, "AND"),
            Token::EQ(_) => write!(f, "=="),
            Token::NE(_) => write!(f, "!="),
            Token::LT(_) => write!(f, "<"),
            Token::LE(_) => write!(f, "<="),
            Token::GT(_) => write!(f, ">"),
            Token::GE(_) => write!(f, ">="),
            Token::ADD(_) => write!(f, "+"),
            Token::SUB(_) => write!(f, "-"),
            Token::MOD(_) => write!(f, "%"),
            Token::MUL(_) => write!(f, "*"),
            Token::DIV(_) => write!(f, "/"),
            Token::POW(_) => write!(f, "^"),
            Token::DIE(_) => write!(f, "d"),
            Token::LPAREN(_) => write!(f, "("),
            Token::RPAREN(_) => write!(f, ")"),
            Token::LBRACKET(_) => write!(f, "["),
            Token::RBRACKET(_) => write!(f, "]"),
            Token::LBRACE(_) => write!(f, "{{"),
            Token::RBRACE(_) => write!(f, "}}"),
            _ => write!(f, "{:?}", self),
        }
    }
}
