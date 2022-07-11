use std::str::FromStr;

use lalrpop_util::ParseError as LalrpopError;
use logos::{Lexer, Logos, SpannedIter};

use crate::kismet::__ToTriple;

pub enum LexerError {
    RANGE,
}

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

    #[regex(r"[0-9]+", parse_int)]
    Int(i32),

    #[regex(r"\$[_a-zA-Z][_a-zA-Z0-9]*")]
    Id(&'input str),
}

fn parse_int<'input>(t: &mut Lexer<'input, Token<'input>>) -> Option<i32> {
    match i32::from_str(t.slice()) {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

pub struct KismetLexer<'input> {
    curr: SpannedIter<'input, Token<'input>>,
}

type Span<'input> = (usize, Token<'input>, usize);
type ParseError<'input> = LalrpopError<usize, Token<'input>, &'static str>;

impl<'input> Iterator for KismetLexer<'input> {
    type Item = Result<Span<'input>, ParseError<'input>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr.next();
        match next {
            Some((Token::ERROR, _)) => Some(Err(ParseError::<'input>::User {
                error: "Lexer error",
            })),
            Some((t, r)) => match t {
                _ => Some(Ok((r.start, t, r.end))),
            },
            None => None,
        }
    }
}

impl<'input> __ToTriple<'input> for Result<Span<'input>, ParseError<'input>> {
    fn to_triple(value: Self) -> Result<(usize, Token<'input>, usize), ParseError<'input>> {
        value
    }
}

pub fn lex<'input>(input: &'input str) -> KismetLexer<'input> {
    let lex = Token::lexer(input);
    KismetLexer {
        curr: lex.spanned(),
    }
}
