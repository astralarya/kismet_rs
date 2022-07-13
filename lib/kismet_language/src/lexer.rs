use std::{fmt, ops::Range};

use logos::{Logos, SpannedIter};

use super::token::Token;

#[derive(Debug, PartialEq)]
pub struct LexerError {
    loc: Range<usize>,
}

pub struct KismetLexer<'input> {
    curr: SpannedIter<'input, Token<'input>>,
}

impl<'input> Iterator for KismetLexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), LexerError>;

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
        write!(f, "Lexer error at {:?}", self.loc)
    }
}

pub fn lex<'input>(input: &'input str) -> KismetLexer<'input> {
    KismetLexer {
        curr: Token::lexer(input).spanned(),
    }
}
