use std::fmt;

use super::{Atom, Node};

#[derive(Debug, PartialEq)]
pub enum Primary<'input> {
    Die(Node<Atom<'input>>),
    Attribute(Node<Primary<'input>>, Node<&'input str>),
    Atom(Atom<'input>),
}

impl fmt::Display for Primary<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primary::Die(val) => match *val.kind {
                Atom::Integer(_) | Atom::Tuple(_) | Atom::ListDisplay(_) => {
                    write!(f, "d{}", val)
                }
                _ => write!(f, "d({})", val),
            },
            Primary::Attribute(lhs, rhs) => write!(f, "{}.{}", lhs, rhs),
            Primary::Atom(val) => write!(f, "{}", val),
        }
    }
}
