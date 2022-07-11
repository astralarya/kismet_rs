use std::ops::Range;
use std::str::FromStr;

use lalrpop_util::ParseError;
use logos::{Logos, SpannedIter};

use crate::kismet::__ToTriple;

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    #[error]
    ERROR,

    #[regex(r"(?i)d")]
    DIE,

    #[token("(")]
    LPAREN,

    #[token(")")]
    RPAREN,

    #[token("^")]
    POW,

    #[token("%")]
    MOD,

    #[token("*")]
    MUL,

    #[token("/")]
    DIV,

    #[token("+")]
    ADD,

    #[token("-")]
    SUB,

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

    #[regex(r"(?i)and")]
    AND,

    #[regex(r"(?i)or")]
    OR,

    #[regex(r"[0-9]+", |s| i32::from_str(s.slice()))]
    Int(i32),

    #[regex(r"\$[_a-zA-Z][_a-zA-Z0-9]*")]
    Id(&'input str),
}

pub struct Lexer<'input> {
    curr: SpannedIter<'input, Token<'input>>,
}

pub enum LexerError {}

type Span<'input> = (Token<'input>, Range<usize>);
type ErrorSpan = (LexerError, Range<usize>);

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Span<'input>, ErrorSpan>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr.next();
        match next {
            Some(t) => Some(Ok(t)),
            None => None,
        }
    }
}

impl<'input> __ToTriple<'input> for Result<Span<'input>, ErrorSpan> {
    fn to_triple(
        value: Self,
    ) -> Result<(usize, Token<'input>, usize), ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok((t, r)) => Ok((r.start, t, r.end)),
            Err(_) => Err(ParseError::User { error: "" }),
        }
    }
}

pub fn lex<'input>(input: &'input str) -> Lexer<'input> {
    let lex = Token::lexer(input);
    Lexer {
        curr: lex.spanned(),
    }
}

/*
INT: Node<'input> = {
  <s:r"[0-9]+"> =>? i32::from_str(s)
    .map(|r| Node::Int(r))
    .map_err(|_| ParseError::User{error: "Integer out of range"}),
}
*/
