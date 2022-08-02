use nom::{
    error::{ContextError, ParseError},
    Needed,
};

use crate::{
    ast::{Expr, TargetDictItem, TargetExpr, TargetKind, TargetListItem},
    types::{Node, ONode, Span},
};

use super::Input;

#[derive(Debug, PartialEq)]
pub enum Error<'input> {
    Error(ErrorKind),
    Convert(Input<'input>, ConvertKind),
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Runtime,
    Eof,
    Lex,
    Incomplete(Needed),
    Nom(nom::error::ErrorKind),
    Predicate,
    Grammar,
    Chain(ONode<ErrorKind>, Box<ErrorKind>),
    Context(ONode<ErrorKind>, String),
    Convert(ConvertKind),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConvertKind {
    ExprBlock(Node<Vec<Node<Expr>>>),
    TargetExpr(TargetExpr),
    TargetKindExpr(Node<TargetKind<TargetExpr>>),
    TargetListItemExpr(Node<TargetListItem<TargetExpr>>),
    TargetDictItemExpr(Node<TargetDictItem<TargetExpr>>),
}

impl<'input> From<Error<'input>> for ErrorKind {
    fn from(val: Error<'input>) -> Self {
        match val {
            Error::Error(x) => x,
            Error::Convert(_, x) => Self::Convert(x),
        }
    }
}

impl<'input> ParseError<Input<'input>> for ONode<Error<'input>> {
    fn from_error_kind(input: Input<'input>, kind: nom::error::ErrorKind) -> Self {
        ONode::new(Span::get0(input), Error::Error(ErrorKind::Nom(kind)))
    }

    fn append(input: Input<'input>, kind: nom::error::ErrorKind, other: Self) -> Self {
        ONode::new(
            Span::get0(input),
            Error::Error(ErrorKind::Chain(
                ONode::<ErrorKind>::convert_from(other),
                Box::new(ErrorKind::Nom(kind)),
            )),
        )
    }
}

impl<'input> ContextError<Input<'input>> for ONode<Error<'input>> {
    fn add_context(input: Input<'input>, ctx: &'static str, other: Self) -> Self {
        ONode::new(
            Span::get0(input),
            Error::Error(ErrorKind::Context(
                ONode::<ErrorKind>::convert_from(other),
                String::from(ctx),
            )),
        )
    }
}
