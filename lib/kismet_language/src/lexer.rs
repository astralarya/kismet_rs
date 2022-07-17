use std::fmt;

use logos::{Lexer as LogosLexer, Logos, SpannedIter};

use crate::token::Token;
use crate::types::Span;

type ParserStream<'input> = Result<(usize, Token<'input>, usize), LexerError>;

pub struct Lexer<'input> {
    lexer: LogosLexer<'input, Token<'input>>,
}

impl<'input> IntoIterator for Lexer<'input> {
    type Item = ParserStream<'input>;

    type IntoIter = LexerIterator<'input>;

    fn into_iter(self) -> Self::IntoIter {
        LexerIterator {
            iter: self.lexer.spanned(),
        }
    }
}

pub struct LexerIterator<'input> {
    iter: SpannedIter<'input, Token<'input>>,
}

impl<'input> Iterator for LexerIterator<'input> {
    type Item = ParserStream<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((Token::ERROR, span)) => Some(Err(LexerError { loc: Span(span) })),
            Some((token, span)) => Some(Ok((span.start, token, span.end))),
            None => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LexerError {
    loc: Span,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer error at {:?}", self.loc)
    }
}

pub fn lex<'input>(input: &'input str) -> Lexer<'input> {
    Lexer {
        lexer: Token::lexer(input),
    }
}
