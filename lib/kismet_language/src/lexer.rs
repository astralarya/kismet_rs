use std::{fmt, ops::Range};

use logos::{Lexer as LogosLexer, Logos, SpannedIter};

use super::token::Token;

type ParserStream<'input> = Result<(usize, Token<'input>, usize), LexerError>;
type SpannedParserStream<'input> = (Range<usize>, ParserStream<'input>);

fn to_parser_stream<'input>(t: Token<'input>, r: Range<usize>) -> ParserStream<'input> {
    match (&t, &r) {
        (Token::ERROR, _) => Err(LexerError { loc: r }),
        _ => Ok((r.start, t, r.end)),
    }
}

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
            Some((t, r)) => Some(to_parser_stream(t, r)),
            None => None,
        }
    }
}

pub struct LexerSpannedIterator<'input> {
    iter: SpannedIter<'input, Token<'input>>,
}

impl<'input> Iterator for LexerSpannedIterator<'input> {
    type Item = SpannedParserStream<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((t, r)) => Some((r.clone(), to_parser_stream(t, r))),
            None => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LexerError {
    loc: Range<usize>,
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
