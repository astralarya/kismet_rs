use std::{fmt, iter::Map};

use logos::{Lexer, Logos};
use syn::{parse_str, LitInt, LitStr};

use super::ast::NodeKind;
use super::types::{Integer, Span};

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    #[regex(r"[;\n]", Token::parse_span)]
    DELIM(Span),

    #[regex(",", Token::parse_span)]
    COMMA(Span),

    #[regex(r"(?i)for", Token::parse_span)]
    FOR(Span),

    #[regex(r"(?i)in", Token::parse_span)]
    IN(Span),

    #[regex(r"(?i)if", Token::parse_span)]
    IF(Span),

    #[regex(r"(?i)or", Token::parse_span)]
    OR(Span),

    #[regex(r"(?i)and", Token::parse_span)]
    AND(Span),

    #[regex(r"(?i)not", Token::parse_span)]
    NOT(Span),

    #[token("==", Token::parse_span)]
    EQ(Span),

    #[token("!=", Token::parse_span)]
    NE(Span),

    #[token("<", Token::parse_span)]
    LT(Span),

    #[token("<=", Token::parse_span)]
    LE(Span),

    #[token(">", Token::parse_span)]
    GT(Span),

    #[token(">=", Token::parse_span)]
    GE(Span),

    #[token("+", Token::parse_span)]
    ADD(Span),

    #[token("-", Token::parse_span)]
    SUB(Span),

    #[token("%", Token::parse_span)]
    MOD(Span),

    #[token("*", Token::parse_span)]
    MUL(Span),

    #[token("/", Token::parse_span)]
    DIV(Span),

    #[token("^", Token::parse_span)]
    POW(Span),

    #[regex(r"(?i)d", Token::parse_span)]
    DIE(Span),

    #[token("(", Token::parse_span)]
    LPAREN(Span),

    #[token(")", Token::parse_span)]
    RPAREN(Span),

    #[token("[", Token::parse_span)]
    LBRACKET(Span),

    #[token("]", Token::parse_span)]
    RBRACKET(Span),

    #[token("{", Token::parse_span)]
    LBRACE(Span),

    #[token("}", Token::parse_span)]
    RBRACE(Span),

    #[regex("\"", Token::parse_string)]
    #[regex("r#*\"", Token::parse_rawstring)]
    String((Span, String)),

    #[regex(r"[[:digit:]][[:digit:]_]*", Token::parse_int)]
    #[regex(r"0b[0-1_]*", Token::parse_int)]
    #[regex(r"0o[0-7_]*", Token::parse_int)]
    #[regex(r"0x[[:xdigit:]_]*", Token::parse_int)]
    Integer((Span, Integer)),

    #[regex(r"([[:alpha:]--[dD]_]|[dD][[:alpha:]_])[[:word:]]*", Token::parse_id)]
    Id((Span, &'input str)),

    #[regex(r"[ \t\f]+", logos::skip)]
    SKIP,

    #[error]
    ERROR,
}

impl<'input> Token<'input> {
    pub fn to_span_iter(
        v: &'input Vec<Token<'input>>,
    ) -> Map<std::slice::Iter<'_, Token<'_>>, fn(&Token) -> Span> {
        v.iter().map(|x| x.span().clone())
    }

    fn parse_span(t: &mut Lexer<'input, Token<'input>>) -> Span {
        Span(t.span())
    }

    fn parse_id(t: &mut Lexer<'input, Token<'input>>) -> (Span, &'input str) {
        (Span(t.span()), t.slice())
    }

    fn parse_int(t: &mut Lexer<'input, Token<'input>>) -> Result<(Span, Integer), ()> {
        match parse_str::<LitInt>(t.slice()) {
            Ok(n) => match n.base10_parse::<Integer>() {
                Ok(i) => Ok((Span(t.span()), i)),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    fn parse_string(t: &mut Lexer<'input, Token<'input>>) -> Result<(Span, String), ()> {
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
                        Ok(n) => Ok((Span(t.span()), n.value())),
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

    fn parse_rawstring(t: &mut Lexer<'input, Token<'input>>) -> Result<(Span, String), ()> {
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
                            Ok(n) => Ok((Span(t.span()), n.value())),
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

    pub fn span(&self) -> &Span {
        match self {
            Token::DELIM(span)
            | Token::COMMA(span)
            | Token::FOR(span)
            | Token::IN(span)
            | Token::IF(span)
            | Token::OR(span)
            | Token::NOT(span)
            | Token::AND(span)
            | Token::EQ(span)
            | Token::NE(span)
            | Token::LT(span)
            | Token::LE(span)
            | Token::GT(span)
            | Token::GE(span)
            | Token::ADD(span)
            | Token::SUB(span)
            | Token::MOD(span)
            | Token::MUL(span)
            | Token::DIV(span)
            | Token::POW(span)
            | Token::DIE(span)
            | Token::LPAREN(span)
            | Token::RPAREN(span)
            | Token::LBRACKET(span)
            | Token::RBRACKET(span)
            | Token::LBRACE(span)
            | Token::RBRACE(span)
            | Token::String((span, _))
            | Token::Integer((span, _))
            | Token::Id((span, _)) => span,
            Token::SKIP | Token::ERROR => &Span(0..0),
        }
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
            Token::FOR(_) => write!(f, "FOR"),
            Token::IN(_) => write!(f, "IN"),
            Token::IF(_) => write!(f, "IF"),
            Token::AND(_) => write!(f, "AND"),
            Token::OR(_) => write!(f, "OR"),
            Token::NOT(_) => write!(f, "NOT"),
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
            Token::String((_, value)) => write!(f, r#""{}""#, value),
            Token::Integer((_, value)) => write!(f, "{}", value),
            Token::Id((_, value)) => write!(f, "{}", value),
            Token::SKIP | Token::ERROR => write!(f, "{:?}", self),
        }
    }
}
