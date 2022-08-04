use std::fmt;

use crate::{
    exec::{Context, Exec1, Value},
    types::{CommaList, Node},
};

use super::{
    Atom, Branch, ExprEnclosure, Id, Loop, Op, Primary, Target, TargetExpr, TargetListItem,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Node<Target>, Node<Expr>),
    Function {
        args: Node<CommaList<TargetListItem<TargetExpr>>>,
        block: Node<ExprEnclosure>,
    },
    Branch(Branch),
    Loop(Loop),
    Op(Op),
    Primary(Primary),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(lhs, rhs) => write!(f, "{} := {}", lhs, rhs),
            Self::Branch(val) => write!(f, "{}", val),
            Self::Loop(val) => write!(f, "{}", val),
            Self::Function { args, block } => {
                write!(f, "({}) => {}", args, block)
            }
            Self::Op(val) => write!(f, "{}", val),
            Self::Primary(val) => write!(f, "{}", val),
        }
    }
}

impl TryFrom<&Node<Expr>> for Node<Id> {
    type Error = ();

    fn try_from(val: &Node<Expr>) -> Result<Self, Self::Error> {
        match &*val.data {
            Expr::Primary(Primary::Atom(Atom::Id(x))) => Ok(Node::new(val.span, Id(x.clone()))),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Node<&Expr>> for Node<Id> {
    type Error = ();

    fn try_from(val: &Node<&Expr>) -> Result<Self, Self::Error> {
        match &*val.data {
            Expr::Primary(Primary::Atom(Atom::Id(x))) => Ok(Node::new(val.span, Id(x.clone()))),
            _ => Err(()),
        }
    }
}

impl Exec1<Context> for Expr {
    type Result = Value;

    fn exec(&self, c: Context) -> (Context, Self::Result) {
        match self {
            Self::Assign(_, _) => todo!(),
            Self::Function { args, block } => todo!(),
            Self::Branch(_) => todo!(),
            Self::Loop(_) => todo!(),
            Self::Op(x) => x.exec(c),
            Self::Primary(x) => x.exec(c),
        }
    }
}
