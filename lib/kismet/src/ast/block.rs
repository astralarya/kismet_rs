use std::{fmt, ops::Deref};

use crate::{
    exec::{Context, Exec, Primitive, Value},
    types::Node,
};

use super::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct ExprTop(pub Vec<Node<Expr>>);

#[derive(Clone, Debug, PartialEq)]
pub struct ExprEnclosure(pub Vec<Node<Expr>>);

impl Deref for ExprTop {
    type Target = Vec<Node<Expr>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ExprEnclosure {
    type Target = Vec<Node<Expr>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ExprTop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Node::join(&self.0, "\n"))
    }
}

impl fmt::Display for ExprEnclosure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {} }}", Node::join(&self.0, "; "))
    }
}

impl Exec<Context> for ExprTop {
    type Result = Value;

    fn exec(&self, mut c: Context) -> (Context, Self::Result) {
        let mut r = Value::Primitive(Primitive::Undefined);
        for x in &self.0 {
            (c, r) = x.exec(c);
        }
        (c, r)
    }
}
