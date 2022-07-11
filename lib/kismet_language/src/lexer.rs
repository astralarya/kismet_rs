use std::{fmt, ops::Range, str::FromStr};

use logos::{Lexer, Logos, SpannedIter};

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
pub enum Token<'input> {
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

    #[regex(r"[ \t\n\f]+", logos::skip)]
    SKIP,

    #[error]
    ERROR,
}

pub struct LexerError {
    loc: Range<usize>,
}

fn parse_int<'input>(t: &mut Lexer<'input, Token<'input>>) -> Result<i32, ()> {
    match i32::from_str(t.slice()) {
        Ok(i) => Ok(i),
        Err(_) => Err(()),
    }
}

pub struct KismetLexer<'input> {
    curr: SpannedIter<'input, Token<'input>>,
}

type Span<'input> = (usize, Token<'input>, usize);

impl<'input> Iterator for KismetLexer<'input> {
    type Item = Result<Span<'input>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr.next();
        match next {
            Some((Token::ERROR, r)) => Some(Err(LexerError { loc: r })),
            Some((t, r)) => Some(Ok((r.start, t, r.end))),
            None => None,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexer error found at {}:{}",
            self.loc.start, self.loc.end
        )
    }
}

pub fn lex<'input>(input: &'input str) -> KismetLexer<'input> {
    let lex = Token::lexer(input);
    KismetLexer {
        curr: lex.spanned(),
    }
}
