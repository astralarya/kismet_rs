use std::fmt;

use logos::{Lexer as LogosLexer, Logos, SpannedIter};

use crate::token::{Token, TokenKind};
use crate::types::Span;

type ParserStream<'input> = Result<(usize, Token<'input>, usize), LexerError>;

pub struct Lexer<'input> {
    lexer: LogosLexer<'input, TokenKind<'input>>,
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
    iter: SpannedIter<'input, TokenKind<'input>>,
}

impl<'input> Iterator for LexerIterator<'input> {
    type Item = ParserStream<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((TokenKind::ERROR, range)) => Some(Err(LexerError { loc: Span(range) })),
            Some((kind, range)) => Some(Ok((
                range.start,
                Token {
                    span: Span(range.clone()),
                    kind,
                },
                range.end,
            ))),
            None => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LexerError {
    pub loc: Span,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer error at {:?}", self.loc)
    }
}

pub fn lex<'input>(input: &'input str) -> Lexer<'input> {
    Lexer {
        lexer: TokenKind::lexer(input),
    }
}
