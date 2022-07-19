pub mod token;
mod types;

use nom::IResult;
pub use types::*;

use token::Token;

pub struct ParseError {}
//type ParseResult<'input> = Result<Node<Expr<'input>>, ParseError>;

pub fn parse<'input>(input: &'input str) -> IResult<Spanned<&'input str>, Spanned<Token<'input>>> {
    token::delim(Spanned::new(input))
}
